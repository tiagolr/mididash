
<script>
import Split from './global/Split.vue';
import GraphView from './graph/GraphView.vue'
import ModulesView from './modules/ModulesView.vue';
import InspView from './inspector/InspView.vue'
import Navbar from './Navbar.vue'
import Window from './global/Window.vue';
import MonitorView from './MonitorView.vue'
import { TOGGLE_MONITOR_IN, TOGGLE_MONITOR_OUT } from '../globals';
import IMonitorIn from '../assets/monitor-in.svg'
import IMonitorOut from '../assets/monitor-out.svg'
import IWrench from '../assets/wrench.svg'
import IBroom from '../assets/broom.svg'
export default {
  components: {
    Split,
    GraphView,
    ModulesView,
    InspView,
    Navbar,
    Window,
    MonitorView,
    IMonitorIn,
    IMonitorOut,
    IWrench,
    IBroom,
  },
  data() {
    return {
      sidebarTab: 'modules',
      monitorIn: {
        enabled: false,
        encoding: 'dec',
        ignoreCols: ['port'],
        filterEvents: [],
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
        x: 10 + 440 + 10,
        y: 50,
        w: 440,
        h: 250,
      },
      windowOnTop: ''
    }
  },
  watch: {
    '$store.graph.selected' (id) {
        this.sidebarTab = id ? 'inspector' : 'modules'
    },
    '$store.app.settings.sidebarWidth' (w) {
      this.$refs.sidebar.offset = w
    },
    '$store.app.settings.monitorIn' (m) {
      this.monitorIn = {...m}
    },
    '$store.app.settings.monitorOut' (m) {
      this.monitorOut = {...m}
    },
  },
  beforeMount () {
    Object.assign(this.monitorIn, this.$store.app.settings.monitorIn) // hot reload only
    Object.assign(this.monitorOut, this.$store.app.settings.monitorOut) // hot reload only
    this.$store.app.emitter.on(TOGGLE_MONITOR_IN, this.toggleMonitorIn)
    this.$store.app.emitter.on(TOGGLE_MONITOR_OUT, this.toggleMonitorOut)
  },
  beforeUnmount () {
    this.$store.app.emitter.off(TOGGLE_MONITOR_IN, this.toggleMonitorIn)
    this.$store.app.emitter.off(TOGGLE_MONITOR_OUT, this.toggleMonitorOut)
  },
  methods: {
    toggleMonitorIn () {
      this.monitorIn.enabled = !this.monitorIn.enabled
      if (this.monitorIn.enabled) {
        this.windowOnTop = 'monitor-in'
      }
      this.saveSettings()
    },
    toggleMonitorOut () {
      this.monitorOut.enabled = !this.monitorOut.enabled
      if (this.monitorOut.enabled) {
        this.windowOnTop = 'monitor-out'
      }
      this.saveSettings()
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
        sidebarWidth: parseInt(this.$refs.sidebar.offset),
      })
    }
  }
}
</script>

<template>
  <div class="main-view">
    <navbar>
    </navbar>
    <split
      ref="sidebar"
      resize-b
      horizontal
      :init="($store.app.settings.sidebarWidth || 250) + 'px'"
      min="100px"
      max="calc(100% - 100px)"
      :gap="2"
      class="overflow"
      @drag-stop="saveSettings"
    >
      <template #A>
        <graph-view class="panel panel-dark">
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
      :style="windowOnTop === 'monitor-in' ? 'z-index: 3' : 'z-index: 2'"
      @close="toggleMonitorIn"
      @mousedown="windowOnTop = 'monitor-in'"
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
      :style="windowOnTop === 'monitor-out' ? 'z-index: 3' : 'z-index: 2'"
      @close="toggleMonitorOut"
      @mousedown="windowOnTop = 'monitor-out'"
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
          @settings-change="s => onMonitorSettingsChange(s, true)"
        ></monitor-view>
      </template>
    </window>
  </div>
</template>

<style scoped>
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