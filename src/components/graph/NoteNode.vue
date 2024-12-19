<script>
import DeviceNode from './DeviceNode.vue';
export default {
  components: {
    DeviceNode
  },
  props: {
    node: Object,
  },
  data() {
    return {
      editable: false
    }
  },
  mounted () {
    document.addEventListener('click', this.onClick)
  },
  beforeUnmount () {
    document.removeEventListener('click', this.onClick)
  },
  methods: {
    onClick () {
      this.$refs.content.blur()
    },
    save () {
      this.$store.graph.setDeviceProperty(this.node.id, 'note', this.$refs.content.innerText)
      this.$store.graph.fitNode(this.node.id)
      this.editable = false
    }
  }
}
</script>

<template>
  <device-node
    :device="node"
    :drag-connector="{}"
    class="note select-none"
  >
    <div class="outer">
      <div
        ref="content"
        class="note-content"
        :contenteditable="editable"
        @click.stop.prevent="editable = true"
        @blur="save"
      >
        {{ node.note }}
      </div>
    </div>
  </device-node>
</template>


<style scoped>
.outer {
  padding: 8px 12px;
}
.note-content {
  outline: none;
  white-space: pre;
  color: var(--text);
}
.note :deep(.content) {
  /* background: #f8f8f8cc; */
}
</style>