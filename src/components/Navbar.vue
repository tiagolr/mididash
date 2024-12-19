<script>
import path from 'path-browserify'
import { TOGGLE_MONITOR_IN, TOGGLE_MONITOR_OUT } from '../globals';
import IMonitorIn from '../assets/monitor-in.svg'
import IMonitorOut from '../assets/monitor-out.svg'
import ContextMenu from './global/ContextMenu.vue'
import IPlay from '../assets/play.svg'
import IPause from '../assets/pause.svg'
import IGear from '../assets/gear.svg'
import IMinimize from '../assets/minimize.svg'
import IMoon from '../assets/moon.svg'
import ISun from '../assets/sun.svg'
export default {
  components: {
    IMonitorIn,
    IMonitorOut,
    ContextMenu,
    IPlay,
    IPause,
    IGear,
    IMinimize,
    IMoon,
    ISun,
  },
  emits: [
    'minimize-to-tray'
  ],
  data() {
    return {
      TOGGLE_MONITOR_IN,
      TOGGLE_MONITOR_OUT,
      mainMenu: false,
    }
  },
  computed: {
    isUserDragging: vm => !!vm.$store.graph.dragdropDevice, // Used to fix glitch caused by drag and drop on linux
    filename () {
      let filename = this.$store.app.settings.projectPath
      if (this.$store.app.os === 'windows' && filename) {
        filename = filename.replaceAll('\\', '/')
      }
      return filename ? path.basename(filename) : ''
    },
    ismacos: vm => vm.$store.app.os === 'macos',
    menuItems: vm => [
      { id: 'new', label: 'New', tip: vm.ismacos ? 'Cmd+N' : 'Ctrl+N'},
      { id: 'new-blank', label: 'New Blank', tip: vm.ismacos ? 'Cmd+Shift+N' : 'Ctrl+Shift+N' },
      { id: 'sep1', type: 'separator'},
      { id: 'open', label: 'Open', tip: vm.ismacos ? 'Cmd+O' : 'Ctrl+O' },
      { id: 'save', label: 'Save', tip: vm.ismacos ? 'Cmd+S' : 'Ctrl+S' },
      { id: 'saveas', label: 'Save As', tip: vm.ismacos ? 'Cmd+Shift+S' : 'Ctrl+Shift+S'},
      { id: 'sep2', type: 'separator'},
      { id: 'about', label: 'About'},
      { id: 'quit', label: 'Quit', tip: vm.ismacos ? 'Cmd+Q' : 'Ctrl+Q'},
    ]
  },
  methods: {
    onSelectMainMenu (id) {
      if (id === 'new') {
        this.$store.app.newDevicesProject()
      } else if (id === 'new-blank') {
        this.$store.app.newBlankProject()
      } else if (id === 'open') {
        this.$store.app.openFile()
      } else if (id === 'save') {
        if (this.$store.app.settings.projectPath) {
          this.$store.app.saveFile()
        } else {
          this.$store.app.saveFileAs()
        }
      } else if (id === 'saveas') {
        this.$store.app.saveFileAs()
      } else if (id === 'settings') {
        this.$store.app.toggleSettings()
      } else if (id === 'about') {
        this.$store.app.toggleAbout()
      } else if (id === 'quit') {
        this.$store.app.forceQuit()
      }
      this.mainMenu = false
    }
  }
}
</script>

<template>
  <div class="navbar select-none" :class="isUserDragging && 'dragging'">
    <div class="left">
      <div class="nav-button" :class="mainMenu && 'active'" @click="mainMenu = !mainMenu">
        <i-gear class="icon" style="transform: scale(0.95)">
        </i-gear>
        <context-menu
          v-if="mainMenu"
          :items="menuItems"
          :start-x="0"
          :start-y="40"
          ignore-padding
          no-shadow
          @select="onSelectMainMenu"
          @close="mainMenu = false"
        ></context-menu>
      </div>
      <div
        class="nav-button"
        :title="$store.app.settings.hubPaused ? 'Start MIDI processing' : 'Pause MIDI processing'"
        @click="$store.app.toggleHubPaused"
      >
        <i-play v-if="$store.app.settings.hubPaused" class="icon" style="transform: scale(0.9)">
        </i-play>
        <i-pause v-else class="icon" style="transform: scale(0.9)">
        </i-pause>
      </div>
      <div class="nav-button" title="Input monitor" @click="$store.app.emitter.emit(TOGGLE_MONITOR_IN)">
        <i-monitor-in class="icon monitor-in">
        </i-monitor-in>
      </div>
      <div class="nav-button" title="Output monitor" @click="$store.app.emitter.emit(TOGGLE_MONITOR_OUT)">
        <i-monitor-out class="icon monitor-out">
        </i-monitor-out>
      </div>
    </div>
    <div class="center">
      <div v-if="filename" class="filename text-ellipsis" :title="$store.app.settings.projectPath">
        {{ filename }}
      </div>
    </div>
    <div class="right">
      <div class="nav-button" @click="$store.app.toggleTheme">
        <i-moon v-if="$store.app.isLightTheme" class="icon">
        </i-moon>
        <i-sun v-else class="icon">
        </i-sun>
      </div>
      <div class="nav-button" title="Minimize to tray" @click="$emit('minimize-to-tray')">
        <i-minimize class="icon">
        </i-minimize>
      </div>
    </div>
  </div>
</template>


<style scoped>
.navbar {
  height: 40px;
  display: flex;
  justify-content: space-between;
  /* border-bottom: 1px solid #222; */
  width: 100%;
  background: var(--foreground);
  cursor: default;
  position: relative;
  user-select: none;
  -webkit-user-select: none;
}
.navbar.dragging .nav-button {
  pointer-events: none;
}
.navbar .nav-button {
  cursor: pointer;
  min-height: 40px;
  min-width: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  user-select: none;
  -webkit-user-select: none;
}
.navbar .nav-button:hover, .navbar .nav-button.active {
  background: var(--foreground-lighter);
}
.nav-button .icon {
  width: 26px;
  height: 26px;
}

:deep(.nav-button .icon path) {
  fill: var(--text) !important;
}
:deep(.nav-button .icon.logo path:first-child) {
  fill: var(--text) !important
}
:deep(.nav-button .icon.logo path:last-child) {
  fill: var(--background) !important;
}
/* :deep(.icon.monitor-in path) {
  fill: var(--error-content) !important;
}
:deep(.icon.monitor-out path) {
  fill: var(--primary-content) !important;
} */
.center {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}
.center, .left, .right {
  display: flex;
  justify-self: stretch;
  align-items: center;
}
.filename {
  font-weight: 600;
  letter-spacing: 1px;
}
</style>