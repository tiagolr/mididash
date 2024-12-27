import { open, save } from '@tauri-apps/plugin-dialog';
import { getVersion } from '@tauri-apps/api/app';
import { defineStore } from 'pinia'
import { invoke } from "@tauri-apps/api/core";
import Emitter from 'tiny-emitter'
import { camelCase, snakeCase } from '../utils';
import graphStore from './graph';
import { stripPrefix } from '../utils';
import { DEFAULT_SCRIPT_TEMPLATES } from '../globals';
import { saveWindowState, StateFlags } from '@tauri-apps/plugin-window-state'

export default defineStore('app', {
  state: () => ({
    emitter: new Emitter(),
    version: '',
    os: __TAURI_OS_PLUGIN_INTERNALS__.os_type, // windows, linux or macos
    showAbout: false, // about popup
    showSettings: false, // settings popup
    settings: { // global settings
      projectPath: '',
      sidebarWidth: 0,
      scriptTemplates: [],
      scriptShowLineNumbers: false,
      hubPaused: false,
      monitorIn: {},
      monitorOut: {}
    },
    preferences: {}, // project settings
    flashMessages: [],
    midiPorts: { // available midi ports fetched from backend and polled for connects/disconnects
      inputs: [],
      outputs: []
    },
    keys: { // keep track of modifiers pressed
      shift: false
    }
  }),
  getters: {
    isLightTheme: vm => vm.settings.theme === 'light'
  },
  actions: {
    async init () {
      this.version = await getVersion()
      await this.getSettings()
      if (!this.settings.scriptTemplates?.length) {
        this.settings.scriptTemplates = JSON.parse(JSON.stringify(DEFAULT_SCRIPT_TEMPLATES))
      } else {
        this.settings.scriptTemplates
          .filter(t => t.id)
          .forEach(t => {
            // update default templates using latest version from DEFAULT_SCRIPT_TEMPLATES
            const index = DEFAULT_SCRIPT_TEMPLATES.find(tt => tt.id === t.id)
            if (index > -1) {
              Object.assign(t, JSON.parse(JSON.stringify(DEFAULT_SCRIPT_TEMPLATES[index])))
            }
          })
      }
      await this.setSettings({})
    },

    async newDevicesProject () {
      await invoke('new_devices_project')
    },

    async newBlankProject () {
      await invoke('new_blank_project')
    },

    async forceQuit () {
      await saveWindowState(StateFlags.ALL)
      await invoke('exit')
    },

    toggleAbout () {
      this.showAbout = !this.showAbout
    },

    toggleSettings () {
      this.showSettings = !this.showSettings
    },

    toggleTheme () {
      this.setSettings({
        theme: (!this.settings.theme || this.settings.theme === 'dark') ? 'light' : 'dark'
      })
    },

    async toggleHubPaused () {
      await invoke('set_hub_paused', { paused: !this.settings.hubPaused })
      await this.getSettings()
      if (this.settings.hubPaused) {
        this.showWarn('MIDI processing paused')
      } else {
        this.showSuccess('MIDI processing resumed')
      }
    },

    async onGlobalEvent (event) {
      this.emitter.emit(event.event, camelCase(event.payload))
    },

    async getSettings () {
      try {
        const settings = await invoke('get_settings')
        this.settings = camelCase(settings)
      } catch (e) {
        this.handleError(e)
      }
    },

    async saveScriptTemplate({ name, outPorts, script }) {
      this.settings.scriptTemplates.push({ name, outPorts, script })
      await this.setSettings()
    },

    async setScriptTemplates(templates) {
      this.settings.scriptTemplates = templates
      await this.setSettings()
    },

    async setSettings (settings = {}) {
      try {
        await invoke('set_settings', { settings: snakeCase(Object.assign(this.settings, settings)) })
        await this.getSettings()
      } catch (e) {
        this.handleError(e)
      }
    },

    async getMidiPorts () {
      try {
        const res = await invoke('get_midi_ports');
        this.midiPorts = res;
        const nodes = graphStore().nodes
        for (let node of nodes) {
          if (
            node.class === 'input' && !this.midiPorts.inputs.includes(stripPrefix(node.id)) ||
            node.class === 'output' && !this.midiPorts.outputs.includes(stripPrefix(node.id))
          ) {
            node.disconnected = true // io not in the list of available ports, disconnect it
          } else if (node.disconnected && (node.class === 'input' || node.class === 'output')) {
            try {
              await invoke('reconnect_device', { id: node.id }) // io now available on ports list, reconnect it
              node.disconnected = false
            } catch (err) {
              this.handleError(err)
            }
          }
        }
      } catch (e) {
        return this.handleError(e);
      }
    },

    async onProjectNew () {
      const graph = graphStore()
      graph.selected = ''
      while (graph.nodes.length) {
        graph.nodes.pop()
      }
      while (graph.edges.length) {
        graph.edges.pop()
      }
      try {
        await this.getSettings() // make sure settings are up to date
        const project = await invoke('get_project').then(camelCase)
        this.preferences = camelCase(project.preferences) || {}

        project.devices.forEach(device => {
          if (typeof device.name === 'string') {
            graph.nodes.push(device)
          } else {
            graph.addNewDevice(device)
          }
        })

        project.connectors.forEach(connector => {
          if (typeof connector.type === 'string') {
            graph.edges.push(connector)
          } else {
            graph.addNewEdge(connector)
          }
        })

      } catch (err) {
        this.handleError(err)
      }
    },

    showFlash (text, type = 'info', timeout = 4500) {
      const message = { text, type, timeout: null }
      this.flashMessages.push(message)
      if (timeout) {
        message.timeout = setTimeout(() => {
          const index = this.flashMessages.indexOf(message)
          if (index > -1) { this.flashMessages.splice(index, 1) }
        }, timeout)
      }
    },

    async saveFileAs() {
      const path = await save({
        filters: [{ name: 'JSON', extensions: ['json'] }],
        defaultPath: 'filename.json'
      })
      if (path) {
        try {
          await invoke('set_project_path', { path })
          await this.saveFile()
        } catch (err) {
          this.handleError(err)
        }
      }
    },

    async saveFile() {
      try {
        await this.saveCurrentProject()
        await invoke('save_project_file') // project file is saved from current session
        await this.getSettings() // project file may have changed
        this.showSuccess("File saved")
      } catch (err) {
        this.handleError(err)
        await invoke('set_project_path', { path: '' })
      }
    },
    /**
     * Saves current session to config file
     */
    async saveCurrentProject() {
      this.preferences.lastSave = (new Date()).toISOString()
      this.preferences.version = this.version
      await invoke('save_current_project', { project: snakeCase({
        preferences: this.preferences,
        devices: graphStore().nodes,
        connectors: graphStore().edges
      })})
    },

    async openFile() {
      const path = await open({
        filters: [{ name: 'JSON', extensions: ['json'] }]
      })
      if (path) {
        try {
          await invoke('open_project', { path })
        } catch (err) {
          this.handleError(err)
        }
      }
    },

    async hubProcess(opts = {ts, bytes, from, to, fromPort, toPort}) {
      try {
        await invoke('hub_process', opts)
      } catch (err) {
        this.handleError(err)
      }
    },

    setShiftKey(bool) {
      this.keys.shift = bool
    },

    showSuccess (text, timeout = 5000) {
      this.showFlash(text, 'success', timeout)
    },

    showInfo (text, timeout = 5000) {
      this.showFlash(text, 'info', timeout)
    },

    showError (text, timeout = 5000) {
      this.showFlash(text, 'error', timeout)
    },

    showWarn (text, timeout = 5000) {
      this.showFlash(text, 'warn', timeout)
    },

    handleError (err) {
      this.showError(err)
      console.error(err)
    }
  }
})