
<script>
import Split from './global/Split.vue';
import GraphView from './graph/GraphView.vue'
import ModulesView from './modules/ModulesView.vue';
import InspView from './inspector/InspView.vue'
import Navbar from './Navbar.vue'
import Window from './global/Window.vue';
import MonitorView from './MonitorView.vue'
import { TOGGLE_MONITOR_IN, TOGGLE_MONITOR_OUT, TOGGLE_CODE_WINDOW, EXPAND_CODE_WINDOW } from '../globals';
import IMonitorIn from '../assets/monitor-in.svg'
import IMonitorOut from '../assets/monitor-out.svg'
import IWrench from '../assets/wrench.svg'
import IBroom from '../assets/broom.svg'
import ICode from '../assets/code.svg'
import CodeView from './CodeView.vue';

export default {
  components: {
    Split,
    GraphView,
    ModulesView,
    InspView,
    Navbar,
    Window,
    MonitorView,
    CodeView,
    IMonitorIn,
    IMonitorOut,
    IWrench,
    IBroom,
    ICode
  },
  emits: [
    'minimize-to-tray'
  ],
  data() {
    return {
      sidebarTab: 'modules',
      monitorIn: {
        enabled: false,
        encoding: 'dec',
        ignoreCols: ['port'],
        filterEvents: [],
        filterChannels: [],
        x: 10,
        y: 50,
        w: 440,
        h: 250,
      },
      monitorOut: {
        enabled: false,
        encoding: 'dec',
        ignoreCols: ['port'],
        filterEvents: [],
        filterChannels: [],
        x: 10 + 440 + 10,
        y: 50,
        w: 440,
        h: 250,
      },
      codeWindow: {
        enabled: false,
        deviceId: '',
        x: 10,
        y: 50,
        w: 500,
        h: 500,
      },
      windowOrder: ['monitorIn', 'monitorOut', 'code'],
    }
  },
  computed: {
    scriptNodes: vm => vm.$store.graph.nodes.filter(n => n.class === 'script')
  },
  watch: {
    '$store.graph.selected' (id) {
        this.sidebarTab = id ? 'inspector' : 'modules'
    },
    '$store.app.settings.sidebarWidth' (w) {
      this.$refs.sidebar.offset = w
    },
    '$store.app.settings.monitorIn' (m) {
      Object.assign(this.monitorIn, JSON.parse(JSON.stringify(m)))
    },
    '$store.app.settings.monitorOut' (m) {
      Object.assign(this.monitorOut, JSON.parse(JSON.stringify(m)))
    },
    '$store.app.settings.codeWindow' (m) {
      Object.assign(this.codeWindow, JSON.parse(JSON.stringify(m)))
    },
    scriptNodes: {
      immediate: true,
      handler (nodes) {
        if (!nodes.map(n => n.id).includes(this.codeWindow.deviceId)) {
          this.codeWindow.deviceId = ''
        }
      }
    },
    'codeWindow.deviceId' () {
      this.saveSettings()
    }
  },
  beforeMount () {
    Object.assign(this.monitorIn, this.$store.app.settings.monitorIn)
    Object.assign(this.monitorOut, this.$store.app.settings.monitorOut)
    Object.assign(this.codeWindow, this.$store.app.settings.codeWindow)
    this.$store.app.emitter.on(TOGGLE_MONITOR_IN, this.toggleMonitorIn)
    this.$store.app.emitter.on(TOGGLE_MONITOR_OUT, this.toggleMonitorOut)
    this.$store.app.emitter.on(TOGGLE_CODE_WINDOW, this.toggleCodeWindow)
    this.$store.app.emitter.on(EXPAND_CODE_WINDOW, this.onExpandCodeWindow)
  },
  beforeUnmount () {
    this.$store.app.emitter.off(TOGGLE_MONITOR_IN, this.toggleMonitorIn)
    this.$store.app.emitter.off(TOGGLE_MONITOR_OUT, this.toggleMonitorOut)
    this.$store.app.emitter.off(TOGGLE_CODE_WINDOW, this.toggleCodeWindow)
    this.$store.app.emitter.off(EXPAND_CODE_WINDOW, this.onExpandCodeWindow)
  },
  methods: {
    bringWindowForward (wname) {
      const index = this.windowOrder.indexOf(wname)
      if (index > -1) {
        this.windowOrder.splice(index, 1)
      }
      this.windowOrder.push(wname)
    },
    toggleMonitorIn () {
      this.monitorIn.enabled = !this.monitorIn.enabled
      if (this.monitorIn.enabled) {
        this.bringWindowForward('monitorIn')
      }
      this.saveSettings()
    },
    toggleMonitorOut () {
      this.monitorOut.enabled = !this.monitorOut.enabled
      if (this.monitorOut.enabled) {
        this.bringWindowForward('monitorOut')
      }
      this.saveSettings()
    },
    toggleCodeWindow () {
      this.codeWindow.enabled = !this.codeWindow.enabled
      if (this.codeWindow.enabled) {
        this.bringWindowForward('code')
      }
      this.saveSettings()
    },
    onExpandCodeWindow (deviceId) {
      this.codeWindow.deviceId = deviceId
      if (!this.codeWindow.enabled) {
        this.toggleCodeWindow()
      }
    },
    onWindowChange () {
      if (this.$refs.monitorOutWin) {
        const rect = this.$refs.monitorOutWin.getRect()
        Object.assign(this.monitorOut, rect)
      }
      if (this.$refs.monitorInWin) {
        const rect = this.$refs.monitorInWin.getRect()
        Object.assign(this.monitorIn, rect)
      }
      if (this.$refs.codeWindow) {
        const rect = this.$refs.codeWindow.getRect()
        Object.assign(this.codeWindow, rect)
      }
      this.saveSettings()
    },
    onMonitorSettingsChange (settings, isMonitorOut) {
      Object.assign(isMonitorOut ? this.monitorOut : this.monitorIn, settings)
      this.saveSettings()
    },
    saveSettings () {
      this.$store.app.setSettings({
        monitorIn: this.monitorIn,
        monitorOut: this.monitorOut,
        codeWindow: this.codeWindow,
        sidebarWidth: parseInt(this.$refs.sidebar.offset),
      })
    }
  }
}
</script>

<template>
  <div class="main-view">
    <navbar @minimize-to-tray="$emit('minimize-to-tray')">
    </navbar>
    <split
      ref="sidebar"
      resize-b
      horizontal
      :init="($store.app.settings.sidebarWidth || 250) + 'px'"
      min="100px"
      max="calc(100% - 100px)"
      :gap="0"
      class="overflow"
      @drag-stop="saveSettings"
    >
      <template #A>
        <graph-view class="panel panel-dark" style="border-radius: 0">
        </graph-view>
      </template>
      <template #B>
        <div class="panel">
          <div class="tabs">
            <div class="tab flex-1" :class="sidebarTab === 'modules' && 'active'" @click="sidebarTab = 'modules'">
              MODULES
            </div>
            <div class="tab flex-1" :class="sidebarTab === 'inspector' && 'active'" @click="sidebarTab = 'inspector'">
              PROPERTIES
            </div>
          </div>
          <div class="panel-inner">
            <modules-view v-if="sidebarTab === 'modules'">
            </modules-view>
            <insp-view v-if="sidebarTab === 'inspector'">
            </insp-view>
          </div>
        </div>
      </template>
    </split>
    <window
      v-if="monitorIn.enabled"
      ref="monitorInWin"
      class="monitor-in"
      :start-x="monitorIn.x"
      :start-y="monitorIn.y"
      :start-w="monitorIn.w"
      :start-h="monitorIn.h"
      :style="`z-index: ${2 + windowOrder.indexOf('monitorIn')}`"
      @close="toggleMonitorIn"
      @mousedown="bringWindowForward('monitorIn')"
      @stop-drag="onWindowChange"
      @stop-resize="onWindowChange"
    >
      <template #header>
        <div class="flex-center flex-1 gap-8 overflow-hidden">
          <i-monitor-in class="icon icon-min">
          </i-monitor-in>
          <div class="text-ellipsis">
            Input monitor
          </div>
          <div class="flex flex-right gap-8">
            <i-broom class="icon cursor-pointer" @click="$refs.monitorIn.clear()">
            </i-broom>
            <i-wrench class="icon cursor-pointer" @click="$refs.monitorIn.toggleSettings()">
            </i-wrench>
          </div>
        </div>
      </template>
      <template #default>
        <monitor-view
          ref="monitorIn"
          in-monitor
          :encoding="monitorIn.encoding"
          :ignore-cols="monitorIn.ignoreCols"
          :filter-events="monitorIn.filterEvents"
          :filter-channels="monitorOut.filterChannels"
          @settings-change="s => onMonitorSettingsChange(s, false)"
        ></monitor-view>
      </template>
    </window>
    <window
      v-if="monitorOut.enabled"
      ref="monitorOutWin"
      class="monitor-out"
      :start-x="monitorOut.x"
      :start-y="monitorOut.y"
      :start-w="monitorOut.w"
      :start-h="monitorOut.h"
      :style="`z-index: ${2 + windowOrder.indexOf('monitorOut')}`"
      @close="toggleMonitorOut"
      @mousedown="bringWindowForward('monitorOut')"
      @stop-drag="onWindowChange"
      @stop-resize="onWindowChange"
    >
      <template #header>
        <div class="flex-center flex-1 gap-8 overflow-hidden">
          <i-monitor-out class="icon icon-mout">
          </i-monitor-out>
          <div class="text-ellipsis">
            Output monitor
          </div>
          <div class="flex flex-right gap-8">
            <i-broom class="icon cursor-pointer" @click="$refs.monitorOut.clear()">
            </i-broom>
            <i-wrench class="icon cursor-pointer" @click="$refs.monitorOut.toggleSettings()">
            </i-wrench>
          </div>
        </div>
      </template>
      <template #default>
        <monitor-view
          ref="monitorOut"
          out-monitor
          :encoding="monitorOut.encoding"
          :ignore-cols="monitorOut.ignoreCols"
          :filter-events="monitorOut.filterEvents"
          :filter-channels="monitorOut.filterChannels"
          @settings-change="s => onMonitorSettingsChange(s, true)"
        ></monitor-view>
      </template>
    </window>
    <window
      v-if="codeWindow.enabled"
      ref="codeWindow"
      class="code-window"
      :start-x="codeWindow.x"
      :start-y="codeWindow.y"
      :start-w="codeWindow.w"
      :start-h="codeWindow.h"
      :style="`z-index: ${2 + windowOrder.indexOf('code')}`"
      @close="toggleCodeWindow"
      @mousedown="bringWindowForward('code')"
      @stop-drag="onWindowChange"
      @stop-resize="onWindowChange"
    >
      <template #header>
        <div class="flex-center flex-1 gap-8 overflow-hidden">
          <i-code class="icon">
          </i-code>
          <select v-model="codeWindow.deviceId" class="select" placeholder="Select node" @mousedown.stop>
            <option :value="''">Select node</option>
            <option v-for="script in scriptNodes" :key="script.id" :value="script.id">
              {{ script.name }}
            </option>
          </select>
        </div>
      </template>
      <template #default>
        <code-view :device="scriptNodes.find(n => n.id === codeWindow.deviceId)">
        </code-view>
      </template>
    </window>
  </div>
</template>

<style scoped>
.select {
  background-color: #fff0;
}
.main-view {
  width: 100%;
  height: 100%;
  position: relative;
  display: flex;
  flex-direction: column;
}
.panel {
  overflow: none;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  /* padding: 10px; */
  border-radius: var(--panel-radius);
  /* background: var(--foreground); */
}
.panel-inner {
  width: 100%;
  height: 100%;
  background: var(--foreground-light);
  display: flex;
  flex-direction: column;
  overflow: auto;
  padding: var(--panel-padding);
}
.tabs {
  display: flex;
}
.tab {
  cursor: pointer;
  text-align: center;
  font-weight: 600;
  /* padding: var(--panel-padding); */
  padding: 8px 14px;
  background: var(--foreground);
  min-width: 70px;
}
.tab.active {
  background: var(--foreground-light);
  color: var(--primary-content);
}
.icon {
  width: 17px;
  height: 17px;
  flex-shrink: 0;
}
.icon.icon-min, .icon.icon-mout {
  width: 19px;
  height: 19px;
}
:deep(.monitor-in .icon.icon-min path) {
  fill: var(--error-content) !important;
}
:deep(.monitor-out .icon.icon-mout path) {
  fill: var(--primary-content) !important;
}
:deep(.icon path) {
  fill: var(--text) !important;
}
.panel-dark {
  background: var(--foreground-dark);
}
:deep(.monitor-in .header) {
  color: var(--error-content)
}
:deep(.monitor-out .header) {
  color: var(--primary-content)
}
</style>