<script>
import { EVT_SCRIPT_ERROR } from '../../globals';
import DeviceNode from './DeviceNode.vue';

export default {
  components: {
    DeviceNode,
  },
  props: {
    node: Object,
    dragConnector: Object
  },
  emits: [
    'mousedown-port',
    'do-connect'
  ],
  created () {
    this.$store.app.emitter.on(EVT_SCRIPT_ERROR, this.onError)
  },
  beforeUnmount () {
    this.$store.app.emitter.off(EVT_SCRIPT_ERROR, this.onError)
  },
  methods: {
    onError(evt) {
      if (evt.id === this.node.id) {
        this.$store.graph.setDeviceProperty(this.node.id, 'error', evt.error)
      }
    }
  }
}
</script>

<template>
  <device-node
    :device="node"
    :drag-connector="dragConnector"
    class="script-node"
    :class="{error: !!node.error}"
    @mousedown-port="args => $emit('mousedown-port', args)"
    @mouseup-port="args => $emit('do-connect', args)"
  >
  </device-node>
</template>


<style scoped>
.script-node.error :deep(.outline) {
  outline: 3px solid red;
}
</style>