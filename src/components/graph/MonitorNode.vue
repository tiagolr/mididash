<script>
import DeviceNode from './DeviceNode.vue';
import MonitorView from '../MonitorView.vue'
import IWrench from '../../assets/wrench.svg'
import IBroom from '../../assets/broom.svg'

export default {
  components: {
    DeviceNode,
    MonitorView,
    IWrench,
    IBroom
  },
  props: {
    node: Object,
    dragConnector: Object
  },
  emits: [
    'mousedown-port',
    'do-connect'
  ],
  data() {
    return {
      settings: this.node.settings || {
        encoding: 'dec',
        ignoreCols: ['time', 'device', 'note', 'eventExt'],
        filterEvents: [],
      }
    }
  },
  methods: {
    onSettingsChange (settings) {
      Object.assign(this.settings, settings)
      this.$store.graph.setDeviceProperty(this.node.id, 'settings', this.settings)
      this.$store.graph.fitNode(this.node.id)
    },
    toggleSettings () {
      this.$refs.monitor.toggleSettings()
    },
    clearMonitor () {
      this.$refs.monitor.clear()
    }
  }
}
</script>

<template>
  <device-node
    :device="node"
    :drag-connector="dragConnector"
    class="monitor-node"
    @mousedown-port="args => $emit('mousedown-port', args)"
    @mouseup-port="args => $emit('do-connect', args)"
  >
    <template #header>
      <div class="flex-center gap-8" style="margin-left: auto">
        <div class="flex-center" title="Clear">
          <i-broom class="icon" @click.stop="clearMonitor">
          </i-broom>
        </div>
        <div title="Settings" class="flex-center">
          <i-wrench class="icon" title="Settings" @click.stop="toggleSettings">
          </i-wrench>
        </div>
      </div>
    </template>
    <div class="monitor-content">
      <monitor-view
        ref="monitor"
        :ignore-cols="settings.ignoreCols"
        :filter-events="settings.filterEvents"
        :encoding="settings.encoding"
        :node="node"
        @settings-change="onSettingsChange"
      ></monitor-view>
    </div>
  </device-node>
</template>


<style scoped>
.monitor-content {
  display: flex;
  flex-direction: column;
  min-height: 150px;
  padding: 8px;
  overflow: auto;
  max-height: 173px;
  height: 173px;
}
.icon {
  width: 18px;
  height: 18px;
  cursor: pointer;
}
.icon:hover :deep(path) {
  fill: var(--foreground-light) !important;
}
</style>