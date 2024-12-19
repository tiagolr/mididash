<script>
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
  data() {
    return {
      keydown: null
    }
  },
  computed: {
    keys: vm => Array
      .from({ length: vm.node.trigger.noteRange * 12})
      .map((_, i) => i + vm.node.trigger.noteStart * 12)
      .filter(i => i < 128),
    keyboardWidth: vm => vm.keys.length * 11 + 'px',
    whiteKeys: vm => vm.keys.filter(k => [0,2,4,5,7,9,11].includes(k % 12)),
    blackKeys: vm => vm.keys.filter(k => [1,3,6,8,10].includes(k % 12)),
    keyWidth: vm => 100 / vm.whiteKeys.length,
    blackKeyWidth: vm => vm.keyWidth * 0.5
  },
  watch: {
    keyboardWidth() {
      this.$store.graph.fitNode(this.node.id)
    }
  },
  created () {
    document.addEventListener('mouseup', this.onMouseupDocument)
  },
  mounted () {
    setTimeout(() => {
      this.$store.graph.fitNode(this.node.id) // FIX ports not aligning correctly
    }, 0);
  },
  beforeUnmount () {
    document.removeEventListener('mouseup', this.onMouseupDocument)
  },
  methods: {
    // returns number of whitekeys before a blackkey
    getBlackSpacing (i) {
      return [1,2,4,5,6][i % 5] + Math.floor(i / 5) * 7
    },
    onMouseupDocument () {
      this.keydown = null
    },
    onMousedownKey (key) {
      this.keydown = key
      this.$store.app.hubProcess({
        ts: Date.now(),
        bytes: [0x90, key, 127],
        from: this.node.id,
        to: '*',
        fromPort: '*',
        toPort: '*'
      })
    },
    onMouseupKey (key) {
      this.keydown = null
      this.$store.app.hubProcess({
        ts: Date.now(),
        bytes: [0x80, key, 0],
        from: this.node.id,
        to: '*',
        fromPort: '*',
        toPort: '*'
      })
    },
    onMouseenterKey (event, key) {
      if (event.buttons === 1) {
        this.onMousedownKey(key)
      }
    },
    onMouseleaveKey (event, key) {
      if (event.buttons === 1) {
        this.onMouseupKey(key)
      }
    }
  }
}
</script>

<template>
  <device-node
    :device="node"
    :drag-connector="dragConnector"
    @mousedown-port="args => $emit('mousedown-port', args)"
    @mouseup-port="args => $emit('do-connect', args)"
  >
    <div class="keys" @click.stop>
      <div class="white-keys">
        <div
          v-for="key in whiteKeys" :key="key"
          class="key white"
          :class="keydown === key && 'keydown'"
          :style="{width: keyWidth + '%'}"
          @mousedown.stop="onMousedownKey(key)"
          @mouseup.stop="onMouseupKey(key)"
          @mouseenter="e => onMouseenterKey(e, key)"
          @mouseleave="e => onMouseleaveKey(e, key)"
        ></div>
      </div>
      <div
        v-for="key,i in blackKeys" :key="key"
        class="key black"
        :class="keydown === key && 'keydown'"
        :style="{
          width: blackKeyWidth + '%',
          left: (getBlackSpacing(i) * keyWidth - blackKeyWidth / 2) + '%'
        }"
        @mousedown.stop="onMousedownKey(key)"
        @mouseup.stop="onMouseupKey(key)"
        @mouseenter="e => onMouseenterKey(e, key)"
        @mouseleave="e => onMouseleaveKey(e, key)"
      ></div>
    </div>
  </device-node>
</template>

<style scoped>
/* :deep(.header) {
  background: none;
}
:deep(.outline) {
  background: var(--node-color);
} */
:deep(.content) {
  background: none;
}
.keys {
  position: relative;
  display: flex;
  margin-left: 1px;
  margin-right: 1px;
  min-width: v-bind(keyboardWidth);
  height: 40px;
}
/* use an extra container div so that first key
  and last key have border radius */
.white-keys {
  display: flex;
  width: 100%;
  height: 100%;
}
.key {
  height: 100%;
  outline: 1px solid var(--trigger-black-keys);
  background: #f8f8f8;
}
.key.white.keydown {
  background: #fcc;
}
.key:first-child {
  border-bottom-left-radius: var(--node-radius);
}
.key.white:last-child {
  border-bottom-right-radius: var(--node-radius);
}
.key.black {
  background: var(--trigger-black-keys);
  width: 6px;
  height: 70%;
  top: -1px;
  position: absolute;
  outline: none;
}
.key.black.keydown {
  background: #933;
}
</style>