use std::collections::HashMap;

use midir::{MidiInput, MidiOutput};
use once_cell::sync::Lazy;
use serde::Serialize;
#[cfg(target_os = "macos")]
use coremidi::{Destinations, Sources};

use crate::globals::PREFIX_INPUT;
use crate::globals::PREFIX_OUTPUT;
#[cfg(not(windows))]
use crate::globals::PREFIX_I;
#[cfg(not(windows))]
use crate::globals::PREFIX_O;
#[cfg(not(windows))]
use crate::globals::PREFIX_VI;
#[cfg(not(windows))]
use crate::globals::PREFIX_VO;
#[cfg(windows)]
static PREFIX_I: &str = PREFIX_INPUT;
#[cfg(windows)]
static PREFIX_O: &str = PREFIX_OUTPUT;
#[cfg(windows)]
static PREFIX_VI: &str = PREFIX_INPUT;
#[cfg(windows)]
static PREFIX_VO: &str = PREFIX_OUTPUT;

#[derive(Debug, Serialize)]
pub struct MidiPorts {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>
}

pub const MIDI_NOTE_OFF: &str = "Note Off";
pub const MIDI_NOTE_ON: &str = "Note On";
pub const MIDI_AFTERTOUCH: & str = "Aftertouch";
pub const MIDI_CC: &str = "CC";
pub const MIDI_PROG_CHNG: &str = "Program";
pub const MIDI_CHANNEL_AT: &str = "Channel AT";
pub const MIDI_PITCH: &str = "Pitch";

pub const MIDI_EXT_SYSEX: &str = "Sysex";
pub const MIDI_EXT_MTC: &str = "MTC";
pub const MIDI_EXT_POSITION: &str = "Position";
pub const MIDI_EXT_SELECT: &str = "Select";
pub const MIDI_EXT_TUNE: &str = "Tune";
pub const MIDI_EXT_SYSEXEND: &str = "SysexEnd";
pub const MIDI_EXT_CLOCK: &str = "Clock";
pub const MIDI_EXT_START: &str = "Start";
pub const MIDI_EXT_CONTINUE: &str = "Continue";
pub const MIDI_EXT_STOP: &str = "Stop";
pub const MIDI_EXT_ACTIVE_SNS: &str = "Active Sns";
pub const MIDI_EXT_RESET: &str = "Rese";

static MIDI_TYPES: Lazy<HashMap<u8, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(0x08, MIDI_NOTE_OFF);
    map.insert(0x09, MIDI_NOTE_ON);
    map.insert(0x0A, MIDI_AFTERTOUCH);
    map.insert(0x0B, MIDI_CC);
    map.insert(0x0C, MIDI_PROG_CHNG);
    map.insert(0x0D, MIDI_CHANNEL_AT);
    map.insert(0x0E, MIDI_PITCH);
    map
});

static MIDI_EXT_TYPES: Lazy<HashMap<u8, &'static str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert(0xF0, MIDI_EXT_SYSEX);
    map.insert(0xF1, MIDI_EXT_MTC);
    map.insert(0xF2, MIDI_EXT_POSITION);
    map.insert(0xF3, MIDI_EXT_SELECT);
    map.insert(0xF6, MIDI_EXT_TUNE);
    map.insert(0xF7, MIDI_EXT_SYSEXEND);
    map.insert(0xF8, MIDI_EXT_CLOCK);
    map.insert(0xFA, MIDI_EXT_START);
    map.insert(0xFB, MIDI_EXT_CONTINUE);
    map.insert(0xFC, MIDI_EXT_STOP);
    map.insert(0xFE, MIDI_EXT_ACTIVE_SNS);
    map.insert(0xFF, MIDI_EXT_RESET);
    map
});

pub fn parse_midi(bytes: Vec<u8>, sysex: &mut Vec<u8>) -> Vec<(&'static str, u8, Vec<u8>)> {
    let mut events = Vec::new();

    if bytes.is_empty() {
        events.push(("Invalid", 0xFF, vec![]));
        return events;
    }

    let status_byte = bytes[0];
    if status_byte == 0xF0 { // Start of a new SysEx message
        sysex.clear();
        sysex.extend_from_slice(&bytes);
        events.push((MIDI_EXT_SYSEX, 0xFF, bytes.clone()));

        if bytes.last() == Some(&0xF7) { // Check if it ends immediately
            events.push((MIDI_EXT_SYSEXEND, 0xFF, sysex.clone()));
            sysex.clear();
        }
    } else if status_byte >= 0xF0 { // Handle extended or real-time MIDI messages
        if let Some(&message_type) = MIDI_EXT_TYPES.get(&status_byte) {
            events.push((message_type, 0xFF, bytes.clone()));
        }
    } else if !sysex.is_empty() { // Continuation of a SysEx message
        let mut is_valid = true;

        for (i, &byte) in bytes.iter().enumerate() {
            if byte >= 0x80 && byte < 0xF8 && (i != bytes.len() - 1 || byte != 0xF7) { // Allow `0xF7` as a valid last byte
                is_valid = false;
                break;
            }
        }

        if is_valid {
            sysex.extend_from_slice(&bytes);
            events.push((MIDI_EXT_SYSEX, 0xFF, bytes.clone()));

            if bytes.last() == Some(&0xF7) { // Check for the end of SysEx
                events.push((MIDI_EXT_SYSEXEND, 0xFF, sysex.clone()));
                sysex.clear();
            }
        } else {
            sysex.clear(); // Invalid SysEx sequence
        }
    } else {
        // Standard MIDI messages
        let message_type = (status_byte & 0xF0) >> 4; // Extract message type
        let channel = status_byte & 0x0F; // Extract channel

        if let Some(&message_name) = MIDI_TYPES.get(&message_type) {
            events.push((message_name, channel, bytes.clone()));
        }
    }

    if events.is_empty() {
        events.push(("Unknown", 0xFF, bytes.clone()));
    }

    events
}
/**
 * Get midi ports that were not created by this application
 */
#[cfg(not(target_os = "macos"))]
pub fn get_valid_midi_ports() -> Result<MidiPorts, Box<dyn std::error::Error>> {
    let input = MidiInput::new("")?;
    let output = MidiOutput::new("")?;
    let in_ports = &input.ports();
    let out_ports = &output.ports();

    let mut res_ins = Vec::new();
    let mut res_outs = Vec::new();

    for p in in_ports {
        let pname = input.port_name(&p)?;
        if !pname.starts_with(PREFIX_OUTPUT) && !pname.starts_with(PREFIX_O) && !pname.starts_with(PREFIX_VO) {
            res_ins.push(pname);
        }
    }

    for p in out_ports {
        let pname = output.port_name(&p)?;
        if !pname.starts_with(PREFIX_INPUT) && !pname.starts_with(PREFIX_I) && !pname.starts_with(PREFIX_VI) {
            res_outs.push(pname);
        }
    }

    Ok(MidiPorts {
        inputs: res_ins,
        outputs: res_outs
    })
}

#[cfg(target_os = "macos")]
pub fn get_valid_midi_ports() -> Result<MidiPorts, Box<dyn std::error::Error>> {
    let mut res_ins = Vec::new();
    let mut res_outs = Vec::new();

    for destination in Sources.into_iter() {
        let pname = destination.display_name().unwrap_or_default();
        if !pname.is_empty() && !pname.starts_with(PREFIX_OUTPUT) && !pname.starts_with(PREFIX_O) && !pname.starts_with(PREFIX_VO) {
            res_ins.push(pname);
        }
    }

    for source in Destinations.into_iter() {
        let pname = source.display_name().unwrap_or_default();
        if !pname.is_empty() && !pname.starts_with(PREFIX_INPUT) && !pname.starts_with(PREFIX_I) && !pname.starts_with(PREFIX_VI) {
            res_outs.push(pname);
        }
    }

    Ok(MidiPorts {
        inputs: res_ins,
        outputs: res_outs
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_midi_note_off () {
        let mut sysex_buffer = Vec::new();
        let res = parse_midi(vec![0x81], &mut sysex_buffer);
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].0, MIDI_NOTE_OFF);
        assert_eq!(res[0].1, 1);
    }

    #[test]
    fn parse_midi_sysex_single () {
        let mut sysex_buffer = Vec::new();
        let packet = vec![0xF0, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0xF7];
        let events = parse_midi(packet, &mut sysex_buffer);
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].0, MIDI_EXT_SYSEX);
        assert_eq!(events[1].0, MIDI_EXT_SYSEXEND);
        assert_eq!(events[0].2, vec![0xF0, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0xF7]);
        assert_eq!(events[1].2, vec![0xF0, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0xF7]);
    }

    #[test]
    fn parse_midi_sysex_seq () {
        let mut sysex_buffer = Vec::new();
        let input_packets = vec![
            vec![0xF0, 0x41, 0x42, 0x43], // sysex start
            vec![0x44, 0x45], // sysex
            vec![0x46, 0xF7], // sysex end
            vec![0x90, 0x45, 0x64], // Note On
        ];

        let mut events = Vec::new();
        for packet in input_packets {
            events.extend_from_slice(&parse_midi(packet, &mut sysex_buffer));
        }
        assert_eq!(events.len(), 5);
        assert_eq!(events[0].0, MIDI_EXT_SYSEX);
        assert_eq!(events[1].0, MIDI_EXT_SYSEX);
        assert_eq!(events[2].0, MIDI_EXT_SYSEX);
        assert_eq!(events[3].0, MIDI_EXT_SYSEXEND);
        assert_eq!(events[4].0, MIDI_NOTE_ON);
        assert_eq!(events[3].2, vec![0xF0, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0xF7]);
    }

    #[test]
    fn parse_midi_sysex_w_clock () {
        let mut sysex_buffer = Vec::new();
        let input_packets = vec![
            vec![0xF0, 0x41, 0x42, 0x43], // sysex start
            vec![0x44, 0x45], // sysex
            vec![0xF8], // clock message
            vec![0x46, 0xF7], // sysex end
            vec![0x90, 0x45, 0x64], // Note On
        ];

        let mut events = Vec::new();
        for packet in input_packets {
            events.extend_from_slice(&parse_midi(packet, &mut sysex_buffer));
        }
        assert_eq!(events.len(), 6);
        assert_eq!(events[0].0, MIDI_EXT_SYSEX);
        assert_eq!(events[1].0, MIDI_EXT_SYSEX);
        assert_eq!(events[2].0, MIDI_EXT_CLOCK);
        assert_eq!(events[3].0, MIDI_EXT_SYSEX);
        assert_eq!(events[4].0, MIDI_EXT_SYSEXEND);
        assert_eq!(events[5].0, MIDI_NOTE_ON);
        assert_eq!(events[4].2, vec![0xF0, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0xF7]);
    }
}