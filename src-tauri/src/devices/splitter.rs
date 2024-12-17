use std::collections::HashMap;
use serde::Serialize;
use serde_json::{Value, Error};
use std::error::Error as StdErr;
use crate::{devices::device::Device, globals::{PORT_AFTERTOUCH, PORT_CC, PORT_CHANNEL_AT, PORT_COMMON, PORT_CONTINUE, PORT_NOTE_OFF, PORT_NOTE_ON, PORT_PITCH, PORT_PROGRAM, PORT_REALTIME, PORT_START, PORT_STOP, PORT_SYSEX, PORT_UNKNOWN}, utils::{parse_midi, MIDI_AFTERTOUCH, MIDI_CC, MIDI_CHANNEL_AT,
    MIDI_EXT_ACTIVE_SNS, MIDI_EXT_CLOCK, MIDI_EXT_CONTINUE, MIDI_EXT_MTC, MIDI_EXT_POSITION,
    MIDI_EXT_RESET, MIDI_EXT_SELECT, MIDI_EXT_START, MIDI_EXT_STOP, MIDI_EXT_SYSEX,
    MIDI_EXT_TUNE, MIDI_NOTE_OFF, MIDI_NOTE_ON, MIDI_PITCH, MIDI_PROG_CHNG
}};

/*
 * Input for midi hardware
 */
#[derive(Serialize)]
pub struct Splitter {
    pub id: String,
    pub class: String,
    #[serde(skip_serializing)]
    pub sysex: HashMap<String, Vec<u8>>
}

impl Splitter {
    pub fn new(id: &str) -> Self {
        Splitter {
            id: String::from(id),
            class: String::from("split"),
            sysex: HashMap::new()
        }
    }
}

impl Device for Splitter {
    fn get_id(&self) -> &str { &self.id }
    fn get_class(&self) -> &str { &self.class }
    fn destroy(&mut self) {}
    fn get_data(&mut self, _key: String) -> Result<Option<Value>, String> { Ok(None) }
    fn set_data(&mut self, _key: String, _data:Value) -> Result<(), String> { Ok(()) }
    fn delete_data(&mut self, _key: String) -> Result<(), String> { Ok(()) }
    fn init(&mut self) -> Result<(), Box<dyn StdErr>> { Ok(()) }

    fn serialize(&self) -> Result<Value, Error> {
        serde_json::to_value(self)
    }
    fn process(
        &mut self,
        bytes: &Vec<u8>,
        from: &str,
        _to: &str,
        from_port: &str,
        _to_port: &str
    ) -> Vec<(String, Vec<u8>)> {
        let src = from.to_string() + from_port;
        if !self.sysex.contains_key(&src) {
            self.sysex.insert(src.clone(), vec![]);
        }
        let mut results = vec![];
        let sysex = self.sysex.get_mut(&src);
        let events = parse_midi(bytes.clone(), sysex.unwrap());
        for event in events {
            let name = event.0;
            let channel = event.1;
            let bytes = event.2;
            let port = match name {
                MIDI_NOTE_OFF => PORT_NOTE_OFF,
                MIDI_NOTE_ON => PORT_NOTE_ON,
                MIDI_AFTERTOUCH => PORT_AFTERTOUCH,
                MIDI_CHANNEL_AT => PORT_CHANNEL_AT,
                MIDI_CC => PORT_CC,
                MIDI_PROG_CHNG => PORT_PROGRAM,
                MIDI_PITCH => PORT_PITCH,
                MIDI_EXT_SYSEX => PORT_SYSEX,
                MIDI_EXT_MTC | MIDI_EXT_POSITION | MIDI_EXT_SELECT | MIDI_EXT_TUNE => PORT_COMMON,
                MIDI_EXT_CLOCK | MIDI_EXT_START | MIDI_EXT_CONTINUE | MIDI_EXT_STOP | MIDI_EXT_ACTIVE_SNS | MIDI_EXT_RESET => PORT_REALTIME,
                _ => PORT_UNKNOWN
            };

            results.push(("*".to_string(), bytes.clone()));
            results.push((port.to_string(), bytes.clone()));

            if port == MIDI_EXT_START {
                results.push((PORT_START.to_string(), bytes.clone()));
            }
            else if port == MIDI_EXT_CONTINUE {
                results.push((PORT_CONTINUE.to_string(), bytes.clone()));
            }
            else if port == MIDI_EXT_STOP {
                results.push((PORT_STOP.to_string(), bytes.clone()));
            }

            if channel != 0xFF {
                results.push(((channel + 1).to_string(), bytes.clone()))
            }
        }


        results
    }
}