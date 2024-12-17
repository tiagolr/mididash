<script>
import { EVT_MIDI } from '../globals';
import { parseMidi, midiNoteName, debounce, midiEventType, throttleWithFinish, MIDI_TYPES, MIDI_EXT_TYPES } from '../utils';
import { CC } from '../midi-data';
import VirtualScroll from './global/VirtualScroll.vue'
import Checkbox from './global/forms/Checkbox.vue';

const MAX_ITEMS = 1000
const columns = {
  time: 'Time',
  dev: 'Dev',
  port: 'Port',
  channel: 'Channel',
  status: 'Status',
  data1: 'Data1',
  data2: 'Data2',
  note: 'Note',
  event: 'Event',
  eventExt: 'Event Ext',
  device: 'Device'
}
export default {
  components: {
    VirtualScroll,
    Checkbox
  },
  props: {
    inMonitor: Boolean, // Monitors all inputs
    outMonitor: Boolean, // Monitors all outputs,
    encoding: {
      type: String,
      default: 'dec',
    },
    node: Object, // if not inMonitor and not outMonitor uses its own node to listen to midi from its connections
    ignoreCols: {
      type: Array,
      default: () => []
    },
    filterEvents: {
      type: Array,
      default: () => []
    }
  },
  emits: [
    'settings-change'
  ],
  data() {
    return {
      midiTypes: Object.values(MIDI_TYPES).concat(Object.values(MIDI_EXT_TYPES)),
      columns,
      sysex: {}, // buffer for sysex messages per device
      items: [],
      itemsQueue: [],
      isScrolledDown: true,
      autoScrolling: false,
      settingsVisible: false,
      settings: {
        encoding: this.hexEncoding ? 'hex' : 'dec',
        ignoreCols: [...this.ignoreCols],
        filterEvents: [...this.filterEvents]
      },
      // updating monitor with every midi input can be overkill for performance
      //
      throttleAppendItemsQueue: throttleWithFinish(this.appendItemsQueue, 100),
      debounceDisableAutoScrolling: debounce(() => {
        this.autoScrolling = false
      }, 100)
    }
  },
  computed: {
    inputs: vm => vm.$store.graph.nodes
      .filter(n => n.class === 'input'),
    outputs: vm => vm.$store.graph.nodes
      .filter(n => n.class === 'output'),
    inputEdges: vm => vm.node && vm.$store.graph.edges
      .filter(e => e.to === vm.node.id),
    filteredItems () {
      if (!this.filterEvents.length) {
        return this.items
      }
      return this.items.filter(i => this.filterEvents.includes(i.event))
    }
  },
  watch: {
    settings: {
      deep: true,
      handler (settings) {
        this.$emit('settings-change', settings)
      }
    }
  },
  beforeMount () {
    this.$store.app.emitter.on(EVT_MIDI, this.onMidi)
  },
  beforeUnmount () {
    this.$store.app.emitter.off(EVT_MIDI, this.onMidi)
  },
  methods: {
    formatTime (date) {
      const hours = date.getHours()
      const minutes = date.getMinutes()
      const seconds = date.getSeconds()
      const milliseconds = date.getMilliseconds()
      const paddedHours = String(hours).padStart(2, '0')
      const paddedMinutes = String(minutes).padStart(2, '0')
      const paddedSeconds = String(seconds).padStart(2, '0')
      const paddedMilliseconds = String(milliseconds).padStart(3, '0')
      return `${paddedHours}:${paddedMinutes}:${paddedSeconds}.${paddedMilliseconds}`
    },
    onMidi ({ from, to, fromPort, bytes }) {
      if (to === '*' && this.outMonitor) {
        const devices = this.$store.graph.edges // fetch nodes connected to this message source
          .filter(e => e.from === from)
          .map(e => this.$store.graph.getNode(e.to))
          .filter(Boolean) // remove devices not found
          .filter((el, idx, arr) => arr.indexOf(el) === idx) // remove duplicates

        devices.forEach(device => {
          this.logMessage({ bytes, from, fromPort, device })
        })
      } else {
        const device = this.$store.graph.getNode((this.inMonitor || this.node) ? from : to)
        this.logMessage({ bytes, from, fromPort, device, to })
      }
    },
    logMessage({ bytes, from, to, fromPort, device }) {
      this.autoScrolling = true
      if (!device || (this.inMonitor && device.class !== 'input') || (this.outMonitor && device.class !== 'output')) { // listening to all inputs or outputs
        return // ignore midi
      }
      if (this.node && !this.inputEdges.some(e => e.from === from && e.fromPort === fromPort && (to === this.node.id || to === '*'))) {
        return // node mode, ignore midi from sources not connected to this monitor node
      }
      // split sysex messages into multiple lines to fit into the virtual scroll
      const splitSysex = (events) => {
        const chunkSize = 8
        const result = []
        for (const event of events) {
          const [name, channel, bytes] = event;
          if (name === 'Sysex') {
            for (let i = 0; i < bytes.length; i += chunkSize) {
              const chunk = bytes.slice(i, i + chunkSize);
              const newEventName = i === 0 ? 'Sysex' : 'Sysexline';
              result.push([newEventName, channel, chunk]);
            }
          } else {
            result.push(event)
          }
        }
        return result
      }

      if (!this.sysex[device.id + fromPort]) {
        this.sysex[device.id + fromPort] = []
      }
      let events = parseMidi(bytes, this.sysex[device.id + fromPort])
      if (events.some(e => e[0] === 'Sysex')) {
        events = splitSysex(events)
      }

      events.forEach(e => {
        let event = e[0]
        const channel = e[1]
        bytes = e[2]
        const type = midiEventType(event)

        // append CC event message
        if (event === 'CC' && !this.ignoreCols.includes('eventExt')) {
          event += ' ' + CC[String(bytes[1])]
        }

        this.itemsQueue.push({
          id: Math.random().toString(36).slice(2),
          type: 'bytes',
          time: this.formatTime(new Date()),
          note: event === 'Note Off' || event === 'Note On' ? midiNoteName(bytes[1]) : '',
          event,
          type,
          deviceShortName: device.name.length > 5 ? device.name.slice(0, 2) + device.name.slice(-3) : device.name,
          deviceName: device.name || '',
          channel,
          fromPortShortName: fromPort.length > 4 ? fromPort.slice(0, 2) + fromPort.slice(-2) : fromPort,
          bytes,
        })
      })

      this.throttleAppendItemsQueue()
      this.debounceDisableAutoScrolling()
    },

    appendItemsQueue() {
      this.items = this.items.concat(this.itemsQueue).slice(-MAX_ITEMS)
      this.itemsQueue = []
      if (this.isScrolledDown) {
        this.$nextTick(() => {
          this.$refs.list.scrollToBottom()
        })
      }
    },

    onScroll () {
      if (this.autoScrolling) return // ignore scroll, this is only for user scroll
      const list = this.$refs.list.$el
      this.isScrolledDown = Math.abs(list.scrollHeight - list.scrollTop - list.clientHeight) < 5;
    },
    formatByte (byte) {
      return byte.toString(this.encoding === 'hex' ? 16 : 10).padStart(3, ' ').toUpperCase()
    },
    clear () {
      this.items = []
    },
    toggleIgnoreSettingsCol (col) {
      this.settings.ignoreCols = this.settings.ignoreCols.includes(col)
        ? this.settings.ignoreCols.filter(c => c !== col)
        : this.settings.ignoreCols.concat(col)
    },
    toggleFilterEvent (name) {
      this.settings.filterEvents = this.settings.filterEvents.includes(name)
        ? this.settings.filterEvents.filter(e => e !== name)
        : this.settings.filterEvents.concat(name)
    },
    toggleSettings () {
      this.settingsVisible = !this.settingsVisible
    }
  }
}
</script>

<template>
  <div class="monitor">
    <div :style="settingsVisible && 'opacity: 0'" style="display: flex; flex-direction: column; overflow-y: auto; overflow-x: hidden" @wheel.stop>
      <div class="row row-header">
        <div v-if="!ignoreCols.includes('time')">
          <pre>TIME        </pre>
        </div>
        <div v-if="!ignoreCols.includes('dev')" title="Device shortname">
          <pre>{{ node ? 'NODE ' : 'DEV  ' }}</pre>
        </div>
        <div v-if="!ignoreCols.includes('port')">
          <pre>PORT</pre>
        </div>
        <div v-if="!ignoreCols.includes('channel')" title="Channel">
          <pre>CH</pre>
        </div>
        <div v-if="!ignoreCols.includes('status')" title="Status">
          <pre>STT</pre>
        </div>
        <div v-if="!ignoreCols.includes('data1')" title="Data 1">
          <pre>DT1</pre>
        </div>
        <div v-if="!ignoreCols.includes('data2')" title="Data 2">
          <pre>DT2</pre>
        </div>
        <div v-if="!ignoreCols.includes('note')">
          <pre>NOTE</pre>
        </div>
        <div v-if="!ignoreCols.includes('event')">
          <pre>{{ ignoreCols.includes('eventExt') ? 'EVENT    ' : 'EVENT        ' }}</pre>
        </div>
        <div v-if="!ignoreCols.includes('device')">
          <pre>DEVICE       </pre>
        </div>
      </div>
      <virtual-scroll ref="list" :items="filteredItems" :item-height="14" :buffer="50" @scroll="onScroll">
        <template #default="{item}">
          <div v-if="item.type !== 'sysex'" class="row" :class="item.type">
            <div v-if="!ignoreCols.includes('time')">
              <pre>{{ item.time }}</pre>
            </div>
            <div v-if="!ignoreCols.includes('dev')">
              <pre>{{ item.deviceShortName.padEnd(5, ' ') }}</pre>
            </div>
            <div v-if="!ignoreCols.includes('port')">
              <pre>{{ item.fromPortShortName.padEnd(4, ' ') }}</pre>
            </div>
            <div v-if="!ignoreCols.includes('channel')">
              <pre>{{ (item.channel === -1 ? '' : String(item.channel + 1)).padStart(2, ' ') }}</pre>
            </div>
            <div v-if="!ignoreCols.includes('status')">
              <pre>{{ item.bytes.length > 0 ? formatByte(item.bytes[0]) : '   ' }}</pre>
            </div>
            <div v-if="!ignoreCols.includes('data1')">
              <pre>{{ item.bytes.length > 1 ? formatByte(item.bytes[1]) : '   ' }}</pre>
            </div>
            <div v-if="!ignoreCols.includes('data2')">
              <pre>{{ item.bytes.length > 2 ? formatByte(item.bytes[2]) : '   ' }}</pre>
            </div>
            <div v-if="!ignoreCols.includes('note')">
              <pre>{{ String(item.note || '').padEnd(4, ' ') }}</pre>
            </div>
            <div v-if="!ignoreCols.includes('event')">
              <pre>{{ item.event.slice(0, ignoreCols.includes('eventExt') ? 9 : 13).padEnd(ignoreCols.includes('eventExt') ? 7 : 13, ' ') }}</pre>
            </div>
            <div v-if="!ignoreCols.includes('device')">
              <pre class="text-ellipsis">{{ item.deviceName }}</pre>
            </div>
          </div>
          <!-- SYSEX MSG -->
          <div v-else class="row" :class="item.type">
            <div v-if="!ignoreCols.includes('time')">
              <pre>{{ item.time }}</pre>
            </div>
            <div v-if="!ignoreCols.includes('dev')">
              <pre>{{ item.deviceShortName.padEnd(5, ' ') }}</pre>
            </div>
            <div v-if="!ignoreCols.includes('port')">
              <pre>{{ item.fromPortShortName.padEnd(4, ' ') }}</pre>
            </div>
            <div>
              <pre>{{ (item.event === 'Sysexline' ? '' : item.event).padEnd(7, ' ') }}</pre>
            </div>
            <div v-if="item.event !== 'SysexEnd'" class="sysex-bytes">
              <div v-for="(byte,i) in item.bytes" :key="i">
                <pre>{{ formatByte(byte) }}</pre>
              </div>
            </div>
          </div>
        </template>
      </virtual-scroll>
    </div>

    <div v-if="settingsVisible" class="settings" @wheel.stop>
      <svg width="19" height="19" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" class="btn-close" @click.stop="settingsVisible = false">
        <path d="M16.875 7.125L7.125 16.875M7.125 7.125L16.875 16.875" stroke="var(--copy-lighter)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
      <div class="font-lighter mb-05rem">Format</div>
      <div class="flex-center gap-4 mb-05rem">
        <input v-model="settings.encoding" type="radio" :value="'dec'" @click.stop>
        <div style="margin-right: 4px;">Decimal</div>
        <input v-model="settings.encoding" type="radio" :value="'hex'" @click.stop>
        <div>Hex</div>
      </div>
      <div class="font-lighter mb-05rem">Columns</div>
      <div class="flex flex-wrap gap-4 mb-05rem">
        <div v-for="(name, col) in columns" :key="col" class="flex gap-4" style="margin-right: 8px" @click.stop>
          <checkbox :checked="!settings.ignoreCols.includes(col)" @click="toggleIgnoreSettingsCol(col)">
          </checkbox>
          <div>{{ name }}</div>
        </div>
      </div>
      <div class="font-lighter mb-05rem">Filter</div>
      <div class="flex flex-wrap gap-4">
        <div v-for="event in midiTypes" :key="event" class="flex gap-4" style="margin-right: 8px" @click.stop>
          <checkbox :checked="settings.filterEvents.includes(event)" @click="toggleFilterEvent(event)">
          </checkbox>
          <div>{{ event }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
:deep(.monitor * ) {
  scroll-behavior: auto;
}
.monitor {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  font-size: 12px;
  overflow: hidden;
  position: relative;
}
.row {
  font-family: "Consolas", "Liberation Mono", "Monaco", monospace;
  user-select: auto;
  -webkit-user-select: auto;
  display: flex;
  gap: 1rem;
}

.settings {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: auto;
  /* background: red; */
}

.btn-close {
  float: right;
  cursor: pointer;
  pointer-events: all;
}

.row.note-on { color: var(--green) }
.row.note-off { color: var(--green-dark) }
.row.cc { color: var(--orange) }
.row.program-change { color: var(--blue) }
.row.after-touch { color: var(--green-yellow) }
.row.pitch { color: var(--red) }
.row.system-common { color: var(--yellow) }

.row.row-header {
  color: var(--copy-lighter);
  padding-bottom: 5px;
  user-select: none;
  -webkit-user-select: none;
  cursor: default;
}
.virtual-scroll {
  overflow-x: hidden;
}
.sysex-bytes {
  display: flex;
}
.sysex-bytes > div {
  margin-right: 8px
}
</style>