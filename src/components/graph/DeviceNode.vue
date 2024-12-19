<script>
import { EVT_MIDI, FIT_NODE } from '../../globals';
import { Port } from '../../lib/vnodes'
import IInput from '../../assets/input.svg'
import IOutput from '../../assets/output.svg'
import IVirtual from '../../assets/virtual.svg'
import IMap from '../../assets/map.svg'
import ISplit from '../../assets/split.svg'
import IDelay from '../../assets/delay.svg'
import IMonitor from '../../assets/monitor.svg'
import INote from '../../assets/note.svg'
import IScript from '../../assets/script.svg'
import ITrigger from '../../assets/piano.svg'

export default {
  components: {
    Port,
    IInput,
    IOutput,
    IVirtual,
    IMap,
    ISplit,
    IDelay,
    IMonitor,
    INote,
    IScript,
    ITrigger,
  },
  props: {
    device: Object,
    dragConnector: Object,
    inline: Boolean // force collapsed mode
  },
  emits: [
    'mousedown-port',
    'mouseup-port'
  ],
  data() {
    return {
      mounted: false,
      hoverInp: null, // using variables and mouseenter to detect hover state while mouse is pressed
      hoverOut: null,
      portActiveTimer: {} // hashmap deactivate port animation timers
    }
  },
  computed: {
    isIO: vm => ['input', 'virtual', 'output'].includes(vm.device.class),
    edgesFrom: vm => vm.$store.graph.edges.filter(e => e.from === vm.device.id),
    edgesTo: vm => vm.$store.graph.edges.filter(e => e.to === vm.device.id),
    inputs: vm => vm.device.inPorts.filter(i => !i.hidden),
    outputs: vm => vm.device.outPorts.filter(i => !i.hidden),
    inlinePorts: vm => vm.inputs.length <= 1 && vm.outputs.length <= 1 && (vm.outputs.length === 0 || vm.outputs[0].id === '*'),
    isCollapsed: vm => (!vm.$slots.default && vm.inlinePorts) || vm.inline,
  },
  created () {
    this.$store.app.emitter.on(EVT_MIDI, this.onMidiEvent)
  },
  mounted () {
    this.$store.graph.emitter.on(FIT_NODE, this.onFitNode)
    setTimeout(() => {
      this.mounted = true // HACK, hides the node while giving time to calculate dimensions and center on mouse cursor
    }, 0);
  },
  beforeUnmount () {
    this.$store.app.emitter.off(EVT_MIDI, this.onMidiEvent)
    this.$store.graph.emitter.off(FIT_NODE, this.onFitNode)
  },
  methods: {
    onFitNode (id) {
      if (id === this.device.id) {
          let parent = this.$parent
          while (parent && !parent.fitContent) {
            parent = parent.$parent
          }
          this.$nextTick(() => {
          parent?.fitContent()
          this.$refs.inport?.forEach(p => {
            p.updatePosition()
          })
          this.$refs.outport?.forEach(p => {
            p.updatePosition()
          })
        })
      }
    },
    onMousedownPort ({ event, inp, out }) {
      const port = inp
        ? this.$refs.inport.find(p => p.id === inp.id)
        : this.$refs.outport.find(p => p.id === out.id)

      this.$emit('mousedown-port', {
        deviceId: this.device.id,
        isInput: !!inp,
        portId: inp?.id || out.id,
        x: this.device.x + port.offset.x,
        y: this.device.y + port.offset.y,
        event
      })
    },
    onMouseupPort ({ inp, out }) {
      this.$emit('mouseup-port', {
        deviceId: this.device.id,
        isInput: !!inp,
        portId: inp?.id || out.id
      })
    },
    isDraggingPort (iid, isInput) {
      return this.dragConnector.active
        && this.dragConnector.isInput === isInput
        && this.dragConnector.node === this.device.id
        && this.dragConnector.port === iid
    },
    onClickNode () {
      const id = this.$store.graph.selected !== this.device.id
        ? this.device.id : ''
      this.$store.graph.select(id)
    },
    onMidiEvent({ from, to, fromPort, toPort }) {
      let port
      if (from === this.device.id) {
        port = this.$refs.outport?.find(p => p.id === fromPort)
      } else if (to === this.device.id || (to === '*' && this.edgesTo.some(e => e.from === from))) {
        port = this.$refs.inport?.find(p => p.id === toPort)
      }
      if (port) {
        const id = from+to+fromPort+toPort
        this.activatePort(port, id)
      }
    },
    onFitPorts (deviceId) {
      if (deviceId === this.device.id) {
        this.$nextTick(() => {
          this.$refs.inport?.forEach(p => {
            p.updatePosition()
          })
          this.$refs.outport?.forEach(p => {
            p.updatePosition()
          })
        })
      }
    },
    activatePort (port, id) {
      const inner = port.$el.querySelector('.port-inner')
      inner.classList.add('active')
      clearTimeout(this.portActiveTimer[id])
      this.portActiveTimer[id] = setTimeout(() => {
        inner.classList.remove('active')
      }, 100);
    }
  }
}
</script>

<template>
  <div
    class="d-node"
    :class="{
      [device.class] : true,
      selected: $store.graph.selected === device.id,
      disconnected: device.disconnected,
      collapsed: isCollapsed,
    }"
    :style="!mounted && `opacity: 0`"
    @click.prevent.stop="onClickNode"
  >
    <div class="outline">
      <div class="header text-ellipsis" :class="{'has-ports-left': inputs.length, 'has-ports-right': outputs.length}">
        <component :is="`i-${device.class}`" class="icon">
        </component>
        <div v-if="device.name">{{ device.name }}</div>
        <slot name="header">
        </slot>
      </div>
      <div class="content">
        <div class="ports" :class="inlinePorts && 'inline'">
          <div class="inputs">
            <div v-for="inp in inputs" :key="inp.id" class="port-container input" :class="inp.id === '*' && 'all'">
              <port
                :id="inp.id"
                ref="inport"
                :edges-to="$store.graph.edges
                  .filter(c => c.to === device.id && c.toPort === inp.id)"
              >
                <div
                  class="port-inner"
                  :class="{
                    connected: isDraggingPort(inp.id, true) || edgesTo.find(e => e.toPort === inp.id),
                    hover: hoverInp === inp.id
                  }"
                  @mouseenter="hoverInp = inp.id"
                  @mouseleave="hoverInp = null"
                  @mousedown.prevent.stop="event => onMousedownPort({ inp, event })"
                  @mouseup.prevent.stop="onMouseupPort({ inp })"
                ></div>
              </port>
              <div class="text-ellipsis port-label">
                {{ inp.name || inp.id }}
              </div>
            </div>
          </div>
          <div class="outputs">
            <div v-for="out in outputs" :key="out.id" class="port-container output" :class="out.id === '*' && 'all'">
              <div class="port-label">
                {{ out.name || out.id }}
              </div>
              <port
                :id="out.id"
                ref="outport"
                :edges-from="$store.graph.edges
                  .filter(c => c.from === device.id && c.fromPort === out.id)"
              >
                <div
                  class="port-inner"
                  :class="{
                    connected: isDraggingPort(out.id, false) || edgesFrom.find(e => e.fromPort === out.id),
                    hover: hoverOut === out.id
                  }"
                  @mouseenter="hoverOut = out.id"
                  @mouseleave="hoverOut = null"
                  @mousedown.prevent.stop="event => onMousedownPort({ event, out })"
                  @mouseup.prevent.stop="onMouseupPort({ out })"
                ></div>
              </port>
            </div>
          </div>
        </div>
        <slot></slot>
      </div>
    </div>
  </div>
</template>


<style scoped>
.d-node {
  height: auto;
  position: relative;
  /*
    Fixes traces/glitches in webkit gtk,
    1px for border + 3px for select outline
  */
  padding: 4px;
}
.d-node.split {
  min-width: 110px;
}
.split .outputs .port-container:first-child * {
  opacity: 0;
  pointer-events: none;
}

.outline {
  border-radius: var(--node-radius);
}
.selected .outline {
  outline: 3px solid var(--edge-color)
}
.header {
  background: var(--node-color);
  color: var(--node-header);
  font-weight: 600;
  padding: 0px 8px;
  height: 32px;
  display: flex;
  gap: 8px;
  align-items: center;
  border-top-left-radius: var(--node-radius);
  border-top-right-radius: var(--node-radius);
  /* border: 1px solid #fff3; */
  /* border-bottom: 1px solid #fff3; */
}
.input .header {
  background: var(--input-color);
}
.output .header {
  background: var(--output-color);
}
.virtual .header {
  background: var(--virtual-color);
}
.split .header {
  background: var(--split-color);
}
.map .header {
  background: var(--map-color);
}
.delay .header {
  background: var(--delay-color);
}
.monitor .header {
  background: var(--monitor-color);
}
.trigger .header {
  background: var(--trigger-color);
}
.note .header {
  background: var(--note-color);
}
.script .header {
  background: var(--script-color);
}

.header .icon {
  width: 19px;
  height: 19px;
  flex-shrink: 0;
  -webkit-user-drag: none;
  user-drag: none;
}

:deep(.header .icon path) {
  fill: var(--node-header) !important;
}

.collapsed .header {
  border-bottom-left-radius: var(--node-radius);
  border-bottom-right-radius: var(--node-radius);
}
.header.has-ports-left {
  padding-left: 15px;
}
.header.has-ports-right {
  padding-right: 15px;
}
.content {
  background: var(--foreground-light-alpha);
  border-bottom-left-radius: var(--node-radius);
  border-bottom-right-radius: var(--node-radius);
}
.collapsed .content {
  background: none;
  border: none;
}
.port-container {
  text-wrap: nowrap;
}
.port-container.all {
  position: absolute;
  top: 4px;
  height: 30px;
  display: flex;
  align-items: center;
}
.port-container.all .port-label {
  display: none;
}
.port-container.input.all {
  left: -1px;
}
.port-container.output.all {
  right: -1px;
}
.ports {
  display: flex;
  padding: 8px;
  margin-left: -13px;
  margin-right: -13px;
}
.ports.inline {
  padding: 0;
}
.port {
  padding: 1px; /* fixes glitches / traces in webkitgtk */
}
.inputs, .outputs {
  flex: 1 0 0;
  max-width: 100%;
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.inputs {
  padding-right: 8px;
}
.outputs {
  padding-left: 8px;
}
.ports .input {
  display: flex;
  align-items: center;
  gap: 4px;
}
.ports .output {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 4px;
}
.port-inner {
  width: var(--port-size);
  height: var(--port-size);
  background: var(--port-color);
  border-radius: 3px;
}
.port-inner.hover {
  background: var(--port-connected-color);
  cursor:crosshair;
}
.port-inner.connected {
  background: var(--port-connected-color);
}
:not(.disconnected) .port-inner.active {
  animation: port-flash 0.1s infinite ease-in;
}
</style>

<style>
@keyframes port-flash {
  0% { background: var(--port-color) }
  0.1% { background: var(--port-active-color); }
  100% { background: var(--port-color); }
}
</style>