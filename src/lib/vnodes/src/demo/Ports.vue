<template>
  <div id="ports-demo" class="demo">
    <div class="viewport">
      <screen ref="screen">
        <edge
          v-for="edge in graph.edges"
          :key="edge.id"
          :data="edge"
          :nodes="graph.nodes"
        ></edge>
        <node v-for="node in graph.nodes" ref="node" :key="node.id" :data="node">
          <div class="node-header"><strong>{{ node.id }}</strong></div>
          <table>
            <td style="border-bottom: none">
              <tr v-for="input in node.inputs" :key="node.id+':'+input">
                <port
                  :id="node.id+':'+input"
                  ref="port"
                  :edges-to="getInputEdges(node, input)"
                >
                  <div
                    class="port-inner"
                    :class="getInputEdges(node, input).length && 'connected'"
                    @mousedown.prevent.stop="evt => startConnect(node, { input }, evt)"
                    @mouseup.prevent.stop="createConnect(node, { input })"
                  >
                  </div>
                </port>
                {{ input.slice(1) }}
              </tr>
            </td>
            <td style="border-bottom: none">
              <tr v-for="output in node.outputs" :key="node.id+':'+output">
                {{ output.slice(1) }}
                <port
                  :id="node.id+':'+output"
                  ref="port"
                  :edges-from="getOutputEdges(node, output)"
                >
                  <div
                    class="port-inner"
                    :class="getOutputEdges(node, output).length && 'connected'"
                    @mousedown.prevent.stop="evt => startConnect(node, { output }, evt)"
                    @mouseup.prevent.stop="createConnect(node, { output })"
                  >
                  </div>
                </port>
              </tr>
            </td>
          </table>
        </node>
      </screen>
    </div>
    <div class="sidebar">
    </div>
  </div>
</template>

<script>
import Screen from '../components/Screen.vue'
import Node from '../components/Node.vue'
import Edge from '../components/Edge.vue'
import Port from '../components/Port.vue'
import graph from '../graph'
export default {
  components: {
    Screen,
    Node,
    Edge,
    Port
  },
  data() {
    return {
      graph: new graph(),
      connecting: null, // { node: {}, input: str, output: str }
      mousePrev: { x: 0, y: 0 },
      zoom: 1
    }
  },
  computed: {
    activeEdge: vm => vm.graph.edges
      .find(e => e.active)
  },
  mounted () {
    this.graph.createNode({
      id: 'a',
      inputs: ['i1'],
      outputs: ['o1', 'o2', 'o3', 'o4']
    })
    this.graph.createNode({
      id: 'b',
      inputs: ['i1', 'i2'],
      outputs: ['o1', 'o2']
    })
    this.graph.createNode({
      id: 'c',
      inputs: ['i1', 'i2'],
      outputs: ['o1', 'o2']
    })
    this.graph.createNode({
      id: 'd',
      inputs: ['i1', 'i2'],
      outputs: ['o1', 'o2']
    })
    this.graph.createEdge({
      from: 'a',
      to: 'b',
      fromPort: 'o1',
      toPort: 'i1',
      active: false,
      type: 'hsmooth'
    })
    this.graph.createEdge({
      from: 'a',
      to: 'c',
      fromPort: 'o1',
      toPort: 'i1',
      active: false,
      type: 'hsmooth'
    })
    this.graph.createEdge({
      from: 'c',
      to: 'd',
      fromPort: 'o1',
      toPort: 'i1',
      active: false,
      type: 'hsmooth'
    })
    this.$nextTick(() => {
      this.graph.graphNodes({ spacing: 75 })
      this.$refs.screen.zoomNodes(this.graph.nodes, { scale: 1 })
    })
    document.addEventListener('mouseup', this.cancelConnect)
    document.addEventListener('mousemove', this.onmousemove)
  },
  beforeUnmount () {
    document.removeEventListener('mouseup', this.cancelConnect)
    document.removeEventListener('mousemove', this.onmousemove)
  },
  methods: {
    startConnect (node, {input, output }, evt) {
      if (this.connecting) return
      const port = this.$refs.port
        .find(p => p.id === `${node.id}:${input || output}`)

      const edge = input && this.getInputEdges(node, input).reverse()[0]
      if (edge) { // edit exiting edge
        edge.active = true
        this.connecting = {
          node: this.graph.nodes.find(n => input ? edge.from === n.id : edge.to === n.id),
          input: output,
          output: input
        }
      } else { // new edge
        this.graph.createEdge({
          from: node.id,
          to: node.id,
          fromPort: input || output,
          toPort: input || output,
          fromAnchor: { ...port.offset },
          toAnchor: { ...port.offset },
          active: true,
          type: 'hsmooth'
        })
        this.connecting = {
          node, input, output
        }
      }

      this.mousePrev = { x: evt.clientX, y: evt.clientY }
      this.zoom = this.$refs.screen.panzoom.getZoom()
    },
    createConnect (node, {input, output}) {
      if (!this.connecting) return
      if (this.isValidConnection({node, input, output}, this.connecting)) {
        if (input) {
          this.activeEdge.to = node.id
          this.activeEdge.toPort = input
        } else if (output) {
          this.activeEdge.from = node.id
          this.activeEdge.fromPort = output
        }
        this.stopConnect()
      } else {
        this.cancelConnect()
      }
    },
    cancelConnect () {
      if (!this.connecting) return
      this.graph.removeEdge(this.activeEdge)
      this.stopConnect()
    },
    stopConnect () {
      if (this.activeEdge) {
        this.activeEdge.active = false
      }
      this.$nextTick(() => {
        this.connecting = null
      })
    },
    isValidConnection (conA, conB) {
      return (conA && conB && conA.node && conB.node)
       && (conA.node !== conB.node)
       && (conA.input && conB.output || conB.input && conA.output)
    },
    // edges that go to this input
    getInputEdges (node, input) {
      return this.graph.edges
        .filter(e => e.to === node.id && e.toPort === input)
    },
    // edges that start from this output
    getOutputEdges (node, output) {
      return this.graph.edges
        .filter(e => e.from === node.id && e.fromPort === output)
    },
    onmousemove (e) {
      if (this.connecting) {
        const offset =  {
          x: (e.clientX - this.mousePrev.x) / this.zoom,
          y: (e.clientY - this.mousePrev.y) / this.zoom
        }
        const anchor = this.connecting.input
          ? this.activeEdge.fromAnchor
          : this.activeEdge.toAnchor

        anchor.x += offset.x
        anchor.y += offset.y
        this.mousePrev = { x: e.clientX, y: e.clientY }
      }
    }
  },
}
</script>

<style scoped>
.port-inner {
  width: 15px;
  height: 15px;
  border-radius: 10px;
  background-color: #abc;
  display: inline-block;
  cursor: crosshair;
}

.port-inner:hover {
  background-color: rgb(2, 219, 67);
}

.port-inner.connected {
  background-color: rgb(2, 219, 67);
}

.node-header {
  text-align: left;
  padding-left: 10px;
  background-color: rgb(40, 150, 95);
  border-radius: 5px 5px 0 0;
  color: rgb(255, 255, 255);
}
</style>

<style>
#ports-demo .node .content {
  background-color: #eee;
  box-shadow: 2px 2px 2px 2px rgb(100, 100, 100, .5);
  /* outline: 2px solid #555; */
}

#ports-demo .edge {
  stroke: rgb(117, 117, 117);
  stroke-linejoin: round;
  marker-start: none;
  marker-end: none;
  stroke-dasharray: 5px 10px;
  stroke-dashoffset: 1000;
  stroke-linecap: round;
  animation: dash 20s linear infinite;
}
</style>

<style>
@keyframes dash {
  to {
    stroke-dashoffset: 0;
  }
}
</style>