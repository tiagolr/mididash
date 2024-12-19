// GLOBAL EVENTS
export const EVT_MIDI = 'midi'
export const EVT_WINDOW_SHOW = 'window-show'
export const EVT_SETTINGS_CHANGE = 'settings-change'
export const EVT_PROJECT_NEW = 'project-new'
export const EVT_PROJECT_CHANGE = 'project-change'
export const EVT_ERROR = 'error'
export const EVT_FILE_OPEN = 'open-file'
export const EVT_FILE_SAVE_AS = 'save-file-as'
export const EVT_FILE_SAVE = 'save-file'
export const EVT_SCRIPT_ERROR = 'script-error'
export const EVT_SCRIPT_LOG = 'script-log'
export const EVT_SHOW_ABOUT = 'show-about'

// APP EVENTS
export const FIT_NODE = 'fit-node'
export const TOGGLE_MONITOR_IN = 'toggle-monitor-in'
export const TOGGLE_MONITOR_OUT = 'toggle-monitor-out'

// splitter outputs ports, see splitter.rs
export const PORT_NOTE_ON = 'noteon';
export const PORT_NOTE_OFF = 'noteoff';
export const PORT_AFTERTOUCH = 'AT';
export const PORT_CC = 'CC';
export const PORT_PROGRAM = 'program';
export const PORT_PITCH = 'pitch';
export const PORT_CHANNEL_AT = 'channelAT';
export const PORT_REALTIME = 'RT';
export const PORT_START = 'start';
export const PORT_CONTINUE = 'continue';
export const PORT_STOP = 'stop';
export const PORT_COMMON = 'CM';
export const PORT_SYSEX = 'sysex';
export const PORT_UNKNOWN = 'unknown';

export const PREFIX_INPUT = 'Mdash In - ';
export const PREFIX_OUTPUT = 'Mdash Out - ';
export const PREFIX_I = 'Mdash In'; // virtual device input
export const PREFIX_O = 'Mdash Out';
export const PREFIX_VI = 'Mdash VIn'; // input for virtual device virtual input
export const PREFIX_VO = 'Mdash VOut';

export const prefixes = [PREFIX_INPUT, PREFIX_OUTPUT, PREFIX_I, PREFIX_O, PREFIX_VI, PREFIX_VO]

export const DEFAULT_PORTS = {
  input: { out: ['*'] },
  output: { in: ['*'] },
  virtual: { in: ['*'], out: ['*'] },
  split: {
    in: ['*'],
    out: [
      '*', '1', '2', '3', '4', '5', '6', '7', '8', '9', '10', '11', '12', '13', '14', '15', '16', PORT_NOTE_ON, PORT_NOTE_OFF, PORT_AFTERTOUCH,
      PORT_CC, PORT_PROGRAM, PORT_PITCH, PORT_CHANNEL_AT, PORT_REALTIME, PORT_START, PORT_CONTINUE, PORT_STOP, PORT_COMMON, PORT_SYSEX, PORT_UNKNOWN
    ],
    visibleOut: [
      '*', '1', '2', PORT_CC, PORT_PROGRAM, PORT_PITCH
    ]},
  map: { in: ['*'], out: ['*'] },
  delay: { in: ['*'], out: ['*'] },
  monitor: { in: ['*'], out: ['*'] },
  note: {},
  trigger: { out: ['*'] },
  script: { in: ['*'] }
}

export const PORT_NAMES = {
  'noteon': 'Note On',
  'noteoff': 'Note Off',
  'AT': 'Aftertouch',
  'CC': 'CC',
  'program': 'Program',
  'pitch': 'Pitch',
  'channelAT': 'ChannelAT',
  'RT': 'Realtime',
  'CM': 'Common',
  'sysex': 'Sysex',
  'start': 'Start',
  'continue': 'Continue',
  'stop': 'Stop',
  '1': 'Channel 1',
  '2': 'Channel 2',
  '3': 'Channel 3',
  '4': 'Channel 4',
  '5': 'Channel 5',
  '6': 'Channel 6',
  '7': 'Channel 7',
  '8': 'Channel 8',
  '9': 'Channel 9',
  '10': 'Channel 10',
  '11': 'Channel 11',
  '12': 'Channel 12',
  '13': 'Channel 13',
  '14': 'Channel 14',
  '15': 'Channel 15',
  '16': 'Channel 16',
}

export const MAPPER_MSGS = [
  { name: 'NoteOff', value: 0x08 },
  { name: 'NoteOn', value: 0x09 },
  { name: 'Afttouch', value: 0x0A },
  { name: 'CC', value: 0x0B },
  { name: 'Program', value: 0x0C },
  { name: 'ChannelAT', value: 0x0D },
  { name: 'Pitch', value: 0x0E }
]

export const DEFAULT_SCRIPT_TEMPLATES = [
  {
    id: 'mdash-basic', // ids identify default templates allowing for updates
    name: 'Basic',
    outPorts: ['out'],
    script:
`-- Prints Lua version
log(_VERSION)

-- Prints incoming bytes
log(bytes)

-- Forward bytes to port "out"
table.insert(res, {
  port="out", bytes=bytes
})

--[[ Globals

log(s): prints to the console
bytes: incomming array of bytes
res: outgoing array of ports and bytes
from: the source node
from_port: the source port
to: the destination node
to_port: the destination port

]]

--[[ Sending bytes

To forward bytes to an output
insert one or more entries
into res:

table.insert(res, {
  port="*", bytes={1,2,3}
})

Forwards bytes [1,2,3] to port "*".
The port must be added manually to the node
otherwise nothing happens.
]]`
  },
  {
    id: 'mdash-filter',
    name: 'Filter',
    outPorts: ['noteon', 'other'],
    script:
`-- ignore empty messages just in case
if #bytes == 0 then return end

-- filter Note ON events to the first port
-- otherwise forward to other port
if bytes[1] >> 4 == 0x09 then
	table.insert(res, {
    port="noteon", bytes=bytes
  })
else
  table.insert(res, {
  	port="other", bytes=bytes
  })
end`
  },
  {
    id: 'mdash-rotator',
    name: 'Rotator',
    outPorts: ['out'],
    script:
`--[[
Rotates the midi channel of messages.
First message on channel 0, next on channel 1,
channel 2 and so on.
]]

-- init global channel
if channel == nil then
	channel = 0
end

-- rotate midi channel
if bytes[1] < 0xF0 then
  bytes[1] = (bytes[1] & 0xF0) | channel
  channel = (channel + 1) % 16
end

-- forward bytes
table.insert(res, {
  port="out", bytes=bytes
})`
  }
]