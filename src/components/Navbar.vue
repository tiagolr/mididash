<script>
import path from 'path-browserify'
import { TOGGLE_MONITOR_IN, TOGGLE_MONITOR_OUT } from '../globals';
import IMonitorIn from '../assets/monitor-in.svg'
import IMonitorOut from '../assets/monitor-out.svg'
import IPlay from '../assets/play.svg'
import IPause from '../assets/pause.svg'
export default {
  components: {
    IMonitorIn,
    IMonitorOut,
    IPlay,
    IPause,
  },
  data() {
    return {
      TOGGLE_MONITOR_IN,
      TOGGLE_MONITOR_OUT,
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
    }
  }
}
</script>

<template>
  <div class="navbar select-none" :class="isUserDragging && 'dragging'">
    <div class="left">
      <div class="nav-button" @click="$store.app.toggleHubPaused">
        <i-play v-if="$store.app.settings.hubPaused" class="icon" style="transform: scale(0.9)">
        </i-play>
        <i-pause v-else class="icon" style="transform: scale(0.9)">
        </i-pause>
      </div>
      <div class="nav-button" @click="$store.app.emitter.emit(TOGGLE_MONITOR_IN)">
        <i-monitor-in class="icon monitor-in">
        </i-monitor-in>
      </div>
      <div class="nav-button" @click="$store.app.emitter.emit(TOGGLE_MONITOR_OUT)">
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
  font-weight: bold;
  cursor: pointer;
  min-height: 40px;
  min-width: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  user-select: none;
  -webkit-user-select: none;
}
.navbar .nav-button:hover {
  background: var(--foreground-lighter);
}
.nav-button .icon {
  width: 26px;
  height: 26px;
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