import { defineStore } from 'pinia'
import { invoke } from "@tauri-apps/api/core";
import { graph } from '../lib/vnodes'
import { appStore } from '.'
import { stripPrefix, camelCase, snakeCase, snakeCaseStr, millisToSecondsStr, capitalize } from '../utils';
import Emitter from 'tiny-emitter'
import { DEFAULT_PORTS, FIT_NODE, PORT_NAMES } from '../globals';

export default defineStore('graph', {
  state: () => ({
    emitter: new Emitter(),
    graph: null,
    nodes: [], // nodes are backend devices with extra properties for the frontend
    edges: [], // same thing for edges are connectors with extra properties
    selected: '',
    dragdropDevice: null, // FIX used to pass data from sidebar via drag and drop as events data is not working
  }),
  getters: {
    nodesMap: vm => vm.nodes.reduce((acc, d) => { acc[d.id] = d; return acc }, {}),
  },
  actions: {
    init () {
      this.graph = new graph()
      this.nodes = this.graph.nodes
      this.edges = this.graph.edges
    },

    select (id) {
      this.selected = id
    },

    getNode (id) {
      return this.nodesMap[id]
    },

    connect(edge = { from, to, fromPort, toPort }) {
      return invoke('connect', edge)
        .then(connector => {
          this.addNewEdge(camelCase(connector))
        })
        .catch(appStore().handleError)
    },

    disconnect(edge = { from, to, fromPort, toPort }) {
      return invoke('disconnect', edge)
        .then(() => {
          edge = this.getEdge(edge)
          this.edges.splice(this.edges.indexOf(edge), 1)
        }).catch(appStore().handleError)
    },

    getEdge({ from, to, fromPort, toPort }) {
      return this.edges.find(c =>
        c.from === from && c.to === to &&
        c.fromPort === fromPort && c.toPort === toPort
      )
    },

    getInputEdges({ to, toPort }) {
      return this.edges.filter(c =>
        c.to === to && c.toPort === toPort
      )
    },

    getOutputEdges({ from, fromPort }) {
      return this.edges.filter(c =>
        c.from === from && c.fromPort === fromPort
      )
    },

    addNewEdge(connector) {
      connector.type = 'hsmooth'
      this.graph.createEdge(connector)
    },

    addNewDevice(device) {
      const node = this.graph.createNode(device)
      node.name = stripPrefix(node.id)
      if (node.class === 'output' || node.class === 'input') {
        const split = node.name.split(':')
        node.name = split[0]
      }
      if (node.class === 'delay') {
        node.name = millisToSecondsStr(node.delay)
      }
      node.inPorts = (DEFAULT_PORTS[node.class]?.in || []).map(id => ({ id, name: PORT_NAMES[id] || id }))
      node.outPorts = (DEFAULT_PORTS[node.class]?.out || []).map(id => ({ id, name: PORT_NAMES[id] || id }))

      // hide default hidden ports
      const visibleOutputs = DEFAULT_PORTS[node.class]?.visibleOut
      if (visibleOutputs) {
        node.outPorts.forEach(p => {
          p.hidden = !visibleOutputs.some(id => id === p.id)
        })
      }

      return node
    },

    async removeDevice(id) {
      const node = this.getNode(id)
      if (!node) {
        console.error('Remove device device not found', id)
        return
      }
      if (this.selected === id) {
        this.selected = ''
      }
      const inputs = this.edges.filter(e => e.to === id)
      const outputs = this.edges.filter(e => e.from === id)
      try {
        for (let i of inputs) {
          await this.disconnect(i)
        }
        for (let o of outputs) {
          await this.disconnect(o)
        }
        if (node.class !== 'note') {
          await invoke('remove_device', { id })
        }
        this.nodes.splice(this.nodes.findIndex(n => n.id === id), 1)
      } catch (err) {
        appStore().handleError(err)
        appStore().onProjectNew()
      }
    },

    async removeSelected() {
      if (this.selected) {
        this.removeDevice(this.selected)
        this.selected = ''
      }
    },

    setDeviceName(id, name) {
      const node = this.getNode(id)
      if (!node) return
      node.name = name.trim()
      this.fitNode(id)
    },

    async createDeviceAt(x, y, centerDevice, opts = { class: 'Unknown' }) {
      const makeUniqueId = classname => {
        const nodes = this.nodes.filter(n => n.class === classname)
        let i = nodes.length + 1
        while (nodes.some(n => n.id === capitalize(classname) + ' ' + i)) { i++ }
        return capitalize(classname) + ' ' + i
      }

      let id = ['input', 'output'].includes(opts.class)
        ? opts.id
        : makeUniqueId(opts.class)

      try {
        let device
        if (opts.class === 'note') {
          device = Object.assign({ inPorts: [], outPorts: [], note: 'Double click to edit' }, opts, { id }) // no need to create notes on the backend
        } else {
          device = await invoke('add_device', { id, class: opts.class })
        }

        device.x = x
        device.y = y
        device = camelCase(device)
        device = this.addNewDevice(device)

        // center new device on mouse position
        if (centerDevice) {
          setTimeout(() => {
            device = this.nodes.at(-1)
            device.x -= device.width / 2
            device.y -= 16
          }, 0);
        }

        // this needs to go some place better
        if (opts.class === 'trigger') {
          device.trigger = {
            noteStart: 5,
            noteRange: 1,
            bytes: '0x90 60 127',
            channel: 0,
            sliders: [{
              id: 'pitch',
              value: 0,
              raw: false,
              visible: false,
              reset: false,
            }]
          }
        }

        // again this should be somewhere else
        if (opts.class === 'map') {
          device.minimized = true
        }

        // probably the same
        if (opts.class === 'script') {
          device.outPorts = opts.outPorts.map(id => ({ id, name: id }))
          device.name = opts.name // template name
          await this.setDeviceData(id, "script", opts.script)
        }

        return device

      } catch (err) {
        appStore().handleError(err)
      }
    },

    fitNode(id) {
      this.emitter.emit(FIT_NODE, id)
    },
    /**
     * Replaces input or output device with another new device from available midi ports list
     */
    async replaceIO(device, id, newname = '') {
      const inputs = this.edges.filter(e => e.from === device.id)
      const outputs = this.edges.filter(e => e.to === device.id)
      try {
        await this.removeDevice(device.id) // remove old device
        const newdevice = await this.createDeviceAt(device.x, device.y, false, { id, class: device.class }) // create new device
        this.setDeviceName(newdevice.id, newname)
        for (let input of inputs) { // reconnect new device
          input.from = newdevice.id
          await this.connect(input)
        }
        for (let output of outputs) { // reconnect new device
          output.to = newdevice.id
          await this.connect(output)
        }
      } catch (err) {
        appStore().handleError(err)
      }
    },

    async setDeviceData(id, key, data) {
      try {
        const device = this.getNode(id)
        if (!device) throw new Error('Device not found')
        await invoke('set_device_data', { id, key: snakeCaseStr(key), data: snakeCase(data) })
        const res = await invoke('get_device', { id })
        Object.assign(device, camelCase(res))
        this.fitNode(id)
      } catch (err) {
        appStore().handleError(err)
        throw err
      }
    },

    async getDeviceData(id, key) {
      try {
        const data = await invoke('get_device_data', {id, key: snakeCaseStr(key) })
        return camelCase(data)
      } catch (err) {
        appStore().handleError(err)
        return null
      }
    },

    async setDeviceProperty(id, key, data) {
      const device = this.getNode(id)
      if (!device) return
      device[key] = data
    },

    /**
     * Toggles port visible
     */
    async toggleOutport(deviceId, portId) {
      const device = this.getNode(deviceId)
      if (!device) return
      const port = device.outPorts.find(p => p.id === portId)
      if (!port) return
      port.hidden = !port.hidden
      if (port.hidden) {
        const edges = this.edges.filter(e => e.from === deviceId && e.fromPort === portId)
        for (const edge of edges) {
          await this.disconnect(edge)
        }
      }
      this.fitNode(deviceId)
    },

    async addOutport(deviceId) {
      const device = this.getNode(deviceId)
      if (!device) return
      let i = device.outPorts.length + 1
      while (device.outPorts.find(port => port.id === `Port ${i}`)) { i++ }
      const id = `Port ${i}`
      device.outPorts.push({ id, name: id })
      this.fitNode(deviceId)
    },

    async removeOutport(deviceId, id) {
      const device = this.getNode(deviceId)
      if (!device) return
      const port = device.outPorts.find(p => p.id === id)
      if (!port) return
      const edges = this.edges.filter(e => e.from === deviceId && e.fromPort === id)
      for (let edge of edges) {
        await this.disconnect(edge)
      }
      device.outPorts.splice(device.outPorts.indexOf(port), 1)
      this.fitNode(deviceId)
    },
    /**
     * Changes port id
     * disconnects all edges from port
     * reconnects all edges to new id
     */
    async setOutportId (deviceId, id, newId) {
      const device = this.getNode(deviceId)
      if (!device) return
      let port = device.outPorts.find(p => p.id === id)
      if (!port) return

      // make unique port id if it already exists
      if (device.outPorts.find(p => p.id === newId && p !== port)) {
        let i = 1
        while (device.outPorts.some(p => `${newId} (${i})` === p.id)) {
          i++
        }
        newId = `${newId} (${i})`
      }
      port.id = newId
      port.name = newId

      // disconnect old edges from output port
      const edgesFrom = this.edges.filter(e => e.from === deviceId && e.fromPort === id)
      for (let edge of edgesFrom) {
        await this.disconnect(edge)
        edge.fromPort = newId
        await this.connect(edge)
      }
      this.fitNode(deviceId)
    },

    setOutputName(deviceId, portId, name) {
      const device = this.getNode(deviceId)
      if (!device) return
      const port = device.outPorts.find(p => p.id === portId)
      if (!port) return
      port.name = name
    },

    layoutNodes () {
      // layout nodes
      const nodePadding = 4 // compensate node padding, nodes have a small padding to fix rendering traces/glitches on linux
      const yspacing = 32 // y spacing between elemenets
      const xspacing = 384 // x spacing between inputs and outputs
      const inputs = this.nodes.filter(n => n.class === 'input')
      const outputs = this.nodes.filter(n => n.class === 'output')
      const others = this.nodes.filter(n => n.class !== 'input' && n.class !== 'output')

      let yacc = -nodePadding
      inputs.forEach(input => {
        input.x = -input.width + nodePadding
        input.y = yacc // distribute inputs vertically
        yacc += input.height + yspacing - nodePadding * 2
      })
      inputs.forEach(input => {
        input.y -= (yacc + nodePadding) / 2 - yspacing // center on x axis
      })

      yacc = -nodePadding
      others.forEach(other => {
        other.x = xspacing / 2 - other.width / 2 - nodePadding / 2
        other.y = yacc
        yacc += other.height + yspacing - nodePadding * 2 // distribute other nodes vertically
      })
      others.forEach(other => {
        other.y -= (yacc + nodePadding) / 2 - yspacing // center on x axis
      })

      yacc = -nodePadding
      outputs.forEach(output => {
        output.x = xspacing - nodePadding
        output.y = yacc // distribute outputs vertically
        yacc += output.height + yspacing - nodePadding * 2
      })
      outputs.forEach(output => {
        output.y -= (yacc + nodePadding) / 2 - yspacing // center on x axis
      })
    }
  }
})