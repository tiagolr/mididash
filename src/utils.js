import { prefixes } from './globals'

export function midiNoteName(noteNumber) {
  const pitchClasses = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
  const pitchClass = noteNumber % 12;
  if (isNaN(pitchClass)) {
    return '    '
  }
  const octave = Math.floor(noteNumber / 12) - 1;
  return `${pitchClasses[pitchClass].padEnd(2, ' ')}${String(octave).padStart(2, ' ')}`;
}

export const MIDI_TYPES = {
  0x08: 'Note Off',
  0x09: 'Note On',
  0x0A: 'Aftertouch',
  0x0B: 'CC',
  0x0C: 'Program',
  0x0D: 'Channel AT',
  0x0E: 'Pitch',
}
export const MIDI_EXT_TYPES = {
  0xF0: 'Sysex',
  0xF1: 'MTC',
  0xF2: 'Position',
  0xF3: 'Select',
  0xF6: 'Tune',
  0xF7: 'SysexEnd', // Sysex end
  0xF8: 'Clock',
  0xFA: 'Start',
  0xFB: 'Continue',
  0xFC: 'Stop',
  0xFE: 'Active Sns', // active sense
  0xFF: 'Reset'
}

const EVENT_TYPES = {
  'Note Off': 'note-off',
  'Note On': 'note-on',
  'Aftertouch': 'after-touch',
  'CC': 'cc',
  'Program': 'program-change',
  'Channel AT': 'after-touch',
  'Pitch': 'pitch',
  'Sysex': 'sysex',
  'MTC': 'system-common',
  'Position': 'system-common',
  'Select': 'system-common',
  'Tune': 'system-common',
  'SysexEnd': 'sysex',
  'Clock': 'real-time',
  'Start': 'real-time',
  'Continue': 'real-time',
  'Stop': 'real-time',
  'Active Sns': 'real-time',
  'Reset': 'real-time',
  'Sysexline': 'sysex'
}

export function midiEventType(event) {
  return EVENT_TYPES[event] || ''
}

export function parseMidi(bytes, sysex = []) {
  const events = [];

  if (bytes.length === 0) {
      events.push(["Invalid", -1, []]);
      return events;
  }

  const statusByte = bytes[0];

  if (statusByte === 0xF0) { // Start of a new SysEx message
      sysex.length = 0;
      sysex.push(...bytes);
      events.push(["Sysex", 0, [...bytes]]);

      if (bytes[bytes.length - 1] === 0xF7) { // Ends immediately
          events.push(["SysexEnd", -1, [...sysex]]);
          sysex.length = 0;
      }
  } else if (statusByte >= 0xF0) { // Handle extended or real-time MIDI messages
      if (MIDI_EXT_TYPES[statusByte]) {
          events.push([MIDI_EXT_TYPES[statusByte], -1, [...bytes]]);
      }
  } else if (sysex.length > 0) { // Continuation of a SysEx message
      let isValid = true;

      for (let i = 0; i < bytes.length; i++) {
          const byte = bytes[i];
          if (byte >= 0x80 && byte < 0xF8 && (i !== bytes.length - 1 || byte !== 0xF7)) {
              isValid = false;
              break;
          }
      }

      if (isValid) {
          sysex.push(...bytes);
          events.push(["Sysex", 0, [...bytes]]);

          if (bytes[bytes.length - 1] === 0xF7) { // Check for the end of SysEx
              events.push(["SysexEnd", -1, [...sysex]]);
              sysex.length = 0;
          }
      } else {
          sysex.length = 0; // Invalid SysEx sequence
      }
  } else { // Standard MIDI messages
      const messageType = (statusByte & 0xF0) >> 4; // Extract message type
      const channel = statusByte & 0x0F; // Extract channel
      if (MIDI_TYPES[messageType]) {
          events.push([MIDI_TYPES[messageType], channel, [...bytes]]);
      }
  }

  if (events.length === 0) {
      events.push(["Unknown", -1, [...bytes]]);
  }

  return events;
}

export function throttle(fn, interval) {
  let lastTime = 0;

  return function (...args) {
    const now = Date.now();
    if (now - lastTime >= interval) {
      lastTime = now;
      fn.apply(this, args);
    }
  }
}

export function millisToSecondsStr(millis) {
  if (millis < 1000) {
    return millis+'ms'
  }
  const seconds = millis / 1000
  return Number.isInteger(seconds) || seconds.toFixed(1).endsWith('.0')
    ? String(Math.round(seconds)) + 's'
    : seconds.toFixed(1) + 's'
}

/**
 * Throttling that always executes the last call
 */
export function throttleWithFinish(fn, interval) {
  let lastTime = 0;
  let timeout;

  return function (...args) {
    const now = Date.now();

    // Call the function immediately if enough time has passed
    if (now - lastTime >= interval) {
      lastTime = now;
      fn.apply(this, args);
    } else {
      // Clear any existing timeout to ensure we schedule the final call
      clearTimeout(timeout);
      timeout = setTimeout(() => {
        lastTime = Date.now();
        fn.apply(this, args);
      }, interval - (now - lastTime));
    }
  }
}

export function debounce(fn, delay) {
  let timer;

  return function (...args) {
    clearTimeout(timer);
    timer = setTimeout(() => {
      fn(...args);
    }, delay);
  };
}

export function capitalize(str) {
  return !str ? '' : str.charAt(0).toUpperCase() + str.slice(1)
}

export function stripPrefix(id) {
  const prefix = prefixes.find(p => id.startsWith(p))
  return prefix ? id.slice(prefix.length) : id
}

export function camelCaseStr(str) {
  return str.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
}

export function camelCase(obj) {
  if (typeof obj !== 'object' || obj === null) {
    return obj;
  }
  if (Array.isArray(obj)) {
    return obj.map(camelCase);
  }
  const result = {};

  for (const key in obj) {
    if (obj.hasOwnProperty(key)) {
      result[camelCaseStr(key)] = camelCase(obj[key]);
    }
  }

  return result;
}

export function snakeCaseStr(str) {
  return str.replace(/([A-Z])/g, (match) => `_${match.toLowerCase()}`);
}

export function snakeCase(obj) {
  if (typeof obj !== 'object' || obj === null) {
    return obj;
  }
  if (Array.isArray(obj)) {
    return obj.map(snakeCase);
  }
  const result = {};

  for (const key in obj) {
    if (obj.hasOwnProperty(key)) {
      result[snakeCaseStr(key)] = snakeCase(obj[key]);
    }
  }

  return result;
}


export function mergeDeep(target, source) {
  if (typeof target !== 'object' || target === null) {
    return source; // If target is not an object, return source
  }
  if (typeof source !== 'object' || source === null) {
    return target; // If source is not an object, return target
  }

  const result = { ...target }; // Clone the target to avoid mutations

  for (const key in source) {
    if (source.hasOwnProperty(key)) {
      if (typeof source[key] === 'object' && source[key] !== null) {
        // Recursively merge objects
        result[key] = mergeDeep(result[key], source[key]);
      } else {
        // Otherwise, directly assign the source value
        result[key] = source[key];
      }
    }
  }

  return result;
}