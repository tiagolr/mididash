<script>
import { Node, Screen, Edge } from '../../lib/vnodes'
import IoGroup from './IOGroup.vue'
import DeviceNode from './DeviceNode.vue'
import MonitorNode from './MonitorNode.vue'
import MapperNode from './MapperNode.vue'
import NoteNode from './NoteNode.vue'
import ScriptNode from './ScriptNode.vue'
import TriggerNode from './TriggerNode.vue'
import { EVT_MIDI } from '../../globals';
import IZoom from '../../assets/zoom.svg'
import ILayout from '../../assets/layout.svg'
import IMagnet from '../../assets/magnet.svg'
export default {
  components: {
    Node,
    Screen,
    Edge,
    IoGroup,
    DeviceNode,
    MonitorNode,
    MapperNode,
    NoteNode,
    ScriptNode,
    TriggerNode,
    IZoom,
    ILayout,
    IMagnet
  },
  data() {
    return {
      unsubscibe: null,
      dragEdge: {
        hiddenEdge: null, // edge hidden when dragging an existing connection
        node: null,
        port: null,
        isInput: false,
        active: false,
        startx: 0, x: 0,
        starty: 0, y: 0,
        mousePrevX: 0,
        mousePrevY: 0,
      },
      pan: {x: 0, y: 0},
      zoom: 1,
      ignoreClicks: false, // workaround, fixes node deselect on connection
      edgeActiveTimer: {}, // hashmap of edge deactivate timers
      gridWidth: window.innerWidth,
      gridHeight: window.innerHeight
    }
  },
  computed: {
    nodes: vm => vm.$store.graph.nodes,
    inputs: vm => vm.nodes.filter(n => n.class === 'input'),
    outputs: vm => vm.nodes.filter(n => n.class === 'output'),
    edges: vm => vm.$store.graph.edges
      .filter(e => e !== vm.dragEdge.hiddenEdge),
    disconnectedEdges: vm => vm.edges
      .filter(e => vm.nodes.some(n => n.disconnected && (n.id === e.from || n.id === e.to))),
    gridY () {
      const lines = []
      const step = 32 * this.zoom
      const totalHeight = this.gridHeight;
      const startY = Math.floor(-this.pan.y / step) * step;
      for (let y = this.pan.y + startY; y < totalHeight; y += step) {
        const isMajorLine = Math.round((y - this.pan.y) / step) % 6 === 0
        if (this.zoom > 0.4 || isMajorLine) {
          lines.push({ y, highlight: this.zoom > 0.4 && isMajorLine });
        }
      }
      return lines
    },
    gridX () {
      const lines = []
      const step = 32 * this.zoom
      const totalWidth = this.gridWidth;
      const startX = Math.floor(-this.pan.x / step) * step;
      for (let x = this.pan.x + startX; x < totalWidth; x += step) {
        const isMajorLine = Math.round((x - this.pan.x) / step) % 6 === 0
        if (this.zoom > 0.4 || isMajorLine) {
          lines.push({ x, highlight: this.zoom > 0.4 && isMajorLine });
        }
      }
      return lines
    },
  },
  created () {
    this.unsubscibe = this.$store.app.$onAction(this.onAction)
    window.addEventListener('resize', this.onWindowResize)
    document.addEventListener('mouseup', this.endConnect)
    this.$store.app.emitter.on(EVT_MIDI, this.onMidiEvent)
  },
  beforeUnmount () {
    document.removeEventListener('mouseup', this.endConnect)
    window.removeEventListener('resize', this.onWindowResize)
    this.unsubscibe()
    this.$store.app.emitter.off(EVT_MIDI, this.onMidiEvent)
  },
  mounted () {
    this.zoomNodes() // focus nodes after show window from minimize to tray
  },
  methods: {
    onWindowResize () {
      this.gridWidth = window.innerWidth
      this.gridHeight = window.innerHeight
    },
    onAction({ name, after }) {
      if (name === 'onProjectNew') {
        after(this.onProjectNew)
      }
    },
    async onProjectNew () {
      await this.$nextTick()
      if (!this.$store.app.preferences.lastSave) { // if its a new project not an opened one
        this.$store.graph.layoutNodes()
      }
      this.zoomNodes()
    },
    zoomNodes () {
      const padding = 50
      if (this.nodes?.length) {
        const rect = this.$refs.screen.zoomNodes(this.nodes, { padding })
        if (this.$refs.screen.getZoom() > 1) {
          this.$refs.screen.zoomRect(rect, { zoom: 1})
        }
      }
    },
    startConnect ({ event, x, y, isInput, deviceId, portId }) {
      if (this.connecting) return
      this.ignoreClicks = true
      this.dragEdge.node = deviceId
      this.dragEdge.port = portId
      this.dragEdge.active = true
      this.dragEdge.startX = x
      this.dragEdge.startY = y
      this.dragEdge.x = x
      this.dragEdge.y = y
      this.dragEdge.isInput = isInput
      this.dragEdge.mousePrevX = event.clientX
      this.dragEdge.mousePrevY = event.clientY
      document.addEventListener('mousemove', this.moveConnect, { passive: true })

      const edge = isInput && this.$store.graph
        .getInputEdges({ to: deviceId, toPort: portId })?.reverse()[0]

      // an existing edge to this port already exists
      // drag existing edge to remove or place it on a different port
      if (edge) {
        this.dragEdge.hiddenEdge = edge
        const device = this.$store.graph.getNode(edge.from)
        const offset = edge.fromAnchor

        this.dragEdge.startX = device.x + offset.x
        this.dragEdge.startY = device.y + offset.y
        this.dragEdge.node = edge.from
        this.dragEdge.port = edge.fromPort
        this.dragEdge.isInput = false
      }
    },
    moveConnect (e) {
      const zoom = this.$refs.screen.getZoom()
      const delta = {
        x: (e.clientX - this.dragEdge.mousePrevX) / zoom,
        y: (e.clientY - this.dragEdge.mousePrevY) / zoom
      }
      this.dragEdge.x += delta.x
      this.dragEdge.y += delta.y
      this.dragEdge.mousePrevX = e.clientX
      this.dragEdge.mousePrevY = e.clientY
    },
    /**
     * Create a connection after dragging from a node port to a node port
     * The dragEdge contains information about the first port selected
     * this method arguments contain information about the second port
     */
    doConnect ({deviceId, portId, isInput}) {
      if (!this.dragEdge.active) return
      let conA = { node: deviceId, port: portId, isInput }
      let conB = {node: this.dragEdge.node, port: this.dragEdge.port, isInput: this.dragEdge.isInput }

      if (isInput) {
        [conA,conB] = [conB,conA] // swap places so first connection is always the output
      }

      if (!conA.isInput && conB.isInput && conA.node !== conB.node) {
        const edge = this.$store.graph.getEdge({
          from: conA.node, fromPort: conA.port,
          to: conB.node, toPort: conB.port
        })
        if (edge) {
          console.error('connection already exists', conA, conB)
          // skip disconnecting edge that is being dragged if the user is connecting it
          // to the same spot it was before
          if (this.dragEdge.hiddenEdge && edge.from === conA.node && edge.to === conB.node &&
           edge.fromPort === conA.port && edge.toPort === conB.port
          ) {
            this.dragEdge.hiddenEdge = null
          }
        } else {
          this.$store.graph.connect({
            from: conA.node,
            to: conB.node,
            fromPort: conA.port,
            toPort: conB.port
          })
        }
      } else {
        console.error('Invalid connection', conA, conB)
      }

      this.endConnect()
    },
    async endConnect () {
      if (!this.dragEdge.active) return
      if (this.dragEdge.hiddenEdge) {
        await this.$store.graph.disconnect(this.dragEdge.hiddenEdge)
      }
      this.dragEdge.active = false
      this.dragEdge.hiddenEdge = null
      document.removeEventListener('mousemove', this.moveConnect)
      setTimeout(() => { this.ignoreClicks = false }, 1);
    },
    isValidConnection (conA, conB) {
      return (conA && conB && conA.node && conB.node)
       && (conA.node !== conB.node)
       && (conA.input && conB.output || conB.input && conA.output)
    },
    /**
     * Drop devices/modules into this view
     */
    onDrop (event) {
      const device = this.$store.graph.dragdropDevice
      if (device) {
        const rect = this.$el.getBoundingClientRect();
        const zoom = this.$refs.screen.getZoom()
        const pan = this.$refs.screen.getPan()
        const x = (event.clientX - rect.left - pan.x) / zoom
        const y = (event.clientY - rect.top - pan.y) / zoom
        this.$store.graph.createDeviceAt(x, y, true, device)
      }
    },

    onClickGraph() {
      if (!this.ignoreClicks) {
        this.$store.graph.select('') // deselect
      }
    },

    onMouseupScreen () {
      setTimeout(() => this.ignoreClicks = false, 1)
    },

    async onMidiEvent({ from, to, fromPort, toPort }) {
      const edges = this.$refs.edge?.filter(e =>
        (e.data.from === from && (e.data.to === to || to === '*')
        && e.data.fromPort === fromPort && e.data.toPort === toPort)
      )
      if (edges?.length) {
        edges.forEach(edge => {
          this.activateEdge(edge)
        })
      }
    },

    activateEdge(edge) {
      edge.$el.classList.add('active')
      clearTimeout(this.edgeActiveTimer[edge.data.id])
      this.edgeActiveTimer[edge.data.id] = setTimeout(() => {
        edge.$el.classList.remove('active')
      }, 100)
    },

    onPan (pan) {
      this.ignoreClicks = true
      this.pan = pan
      this.zoom = this.$refs.screen.getZoom()
    },

    onZoom (zoom) {
      this.zoom = zoom
      this.pan = this.$refs.screen.getPan()
    },

    onDragover (event) {
      event.dataTransfer.dropEffect = 'move' // required to work on windows
    },

    toggleGridSnap () {
      this.$store.app.setSettings({ disableGridSnap: !this.$store.app.settings.disableGridSnap })
    }
  }
}
</script>

<template>
  <div
    class="graph"
    :class="{'dragging': dragEdge.active}"
    @drop.prevent="onDrop"
    @dragover.prevent="onDragover"
    @click="onClickGraph"
  >
    <div class="top-right-buttons" @click.stop>
      <div title="Snap to grid" :class="!$store.app.settings.disableGridSnap && 'active'" @click="toggleGridSnap">
        <i-magnet class="icon">
        </i-magnet>
      </div>
      <div title="Zoom nodes" @click="zoomNodes">
        <i-zoom class="icon">
        </i-zoom>
      </div>
      <div title="Layout nodes" @click="$store.graph.layoutNodes">
        <i-layout class="icon">
        </i-layout>
      </div>
    </div>
    <svg class="grid">
      <line
        v-for="(line, i) in gridY" :key="i"
        :x1="0" x2="100%"
        :y1="line.y" :y2="line.y"
        :class="line.highlight ? 'major-line' : 'line'"
        stroke-width="1"
      ></line>
      <line
        v-for="(line, i) in gridX" :key="i"
        :x1="line.x" :x2="line.x"
        y1="0" y2="100%"
        :class="line.highlight ? 'major-line' : 'line'"
        stroke-width="1"
      ></line>
    </svg>
    <screen ref="screen" class="screen" @dblclick="zoomNodes" @pan="onPan" @zoom="onZoom" @mouseup="onMouseupScreen">
      <template #edges>
        <edge
          v-for="edge in edges" :key="edge.id"
          ref="edge"
          :data="edge"
          :nodes="nodes"
          :style="disconnectedEdges.includes(edge) && 'opacity: 0.4'"
        ></edge>
      </template>
      <template #nodes>
        <io-group
          v-if="inputs.length"
          :nodes="inputs"
          disable-drag
        >
          <div class="group-header">
            Inputs
          </div>
        </io-group>
        <io-group
          v-if="outputs.length"
          :nodes="outputs"
          disable-drag
        >
          <div class="group-header">
            Outputs
          </div>
        </io-group>
        <node
          v-for="node in nodes" :key="node.id"
          :data="node"
          :drag-threshold="2"
          :style="node.disconnected && 'opacity: 0.4'"
          :snap-to="($store.app.settings.disableGridSnap && !$store.app.keys.shift) ||
            (!$store.app.settings.disableGridSnap && $store.app.keys.shift) ? 0 : 16"
        >
          <monitor-node
            v-if="node.class === 'monitor'"
            :drag-connector="dragEdge"
            :node="node"
            @mousedown-port="startConnect"
            @mouseup-port="doConnect"
          ></monitor-node>
          <mapper-node
            v-else-if="node.class === 'map'"
            :drag-connector="dragEdge"
            :node="node"
            @mousedown-port="startConnect"
            @mouseup-port="doConnect"
          ></mapper-node>
          <note-node
            v-else-if="node.class === 'note'"
            :node="node"
          ></note-node>
          <script-node
            v-else-if="node.class === 'script'"
            :drag-connector="dragEdge"
            :node="node"
            @mousedown-port="startConnect"
            @mouseup-port="doConnect"
          ></script-node>
          <trigger-node
            v-else-if="node.class === 'trigger'"
            :drag-connector="dragEdge"
            :node="node"
            @mousedown-port="startConnect"
            @mouseup-port="doConnect"
          ></trigger-node>
          <device-node
            v-else
            :device="node"
            :drag-connector="dragEdge"
            @mousedown-port="startConnect"
            @mouseup-port="doConnect"
          ></device-node>
        </node>
      </template>
      <template #overlay>
        <!--
        New connector uses a placeholder edge
        from node port to mouse coordinates
      -->
        <edge
          v-if="dragEdge.active"
          :data="{
            from: { x: dragEdge.startX, y: dragEdge.startY},
            to: { x: dragEdge.x, y: dragEdge.y },
            type: 'hsmooth'
          }"
        ></edge>
      </template>
    </screen>
  </div>
</template>


<style scoped>
.grid {
  width: 100%;
  height: 100%;
  position: absolute;
  top: 0;
  left: 0;
  z-index: 0;
}
.grid .line {
  stroke: var(--grid-line);
  opacity: 0.9;
}
.grid .major-line {
  stroke: var(--grid-major-line);
  opacity: 0.9;
}
.graph {
  position: relative;
  padding: 0 !important;
  display: flex;
}
.top-right-buttons {
  position: absolute;
  top: 0.5rem;
  right: 0.5rem;
  z-index: 2;
  display: flex;
  align-items: center;
}
.top-right-buttons > div {
  padding: 8px;
  cursor: pointer;
  background: var(--foreground);
  display: flex;
  align-items: center;
}
.top-right-buttons > div:first-child {
  border-top-left-radius: 4px;
  border-bottom-left-radius: 4px;
}
.top-right-buttons > div:last-child {
  border-top-right-radius: 4px;
  border-bottom-right-radius: 4px;
}
.top-right-buttons > div:hover, .top-right-buttons > .active {
  background: var(--foreground-light);
}
.top-right-buttons .icon {
  width: 16px;
  height: 16px;
  cursor: pointer;
}
:deep(.icon path) {
  fill: var(--text) !important;
}
* {
  user-select: none;
  -webkit-user-select: none; /* Safari-specific prefix */
  cursor: default;
}
.graph .screen {
  padding: 0;
  z-index: 1;
}
.node {
  background: none
}
.graph .edge {
  pointer-events: none;
  stroke: var(--edge-color);
  stroke-width: 3px;
  stroke-linejoin: round;
  stroke-linecap: round;
}
.graph .edge.active {
  animation: edge-flash 0.1s infinite ease-in;
}
.group-header {
  width: 100%;
  color: var(--text-lighter);
  user-select: none;
  -webkit-user-drag: none;
}

@keyframes edge-flash {
  0% { stroke: var(--edge-color) }
  0.1% { stroke: var(--edge-active-color); }
  100% { stroke: var(--edge-color); }
}

</style>