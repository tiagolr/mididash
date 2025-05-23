<script>
import DeviceNode from './DeviceNode.vue';
import { TRIGGER_CC_HR, TRIGGER_CC_RAW } from '../../midi-data'
import VueSlider from "vue-3-slider-component"

export default {
  components: {
    DeviceNode,
    VueSlider
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
      keydown: null,
      trigger: this.node.trigger,
      zoom: 1
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
    blackKeyWidth: vm => vm.keyWidth * 0.5,
    triggerSliders: vm => vm.trigger.sliders
      .filter(t => t.visible)
      .map(c => c.raw
        ? TRIGGER_CC_RAW.find(cc => cc.id === c.id)
        : TRIGGER_CC_HR.find(cc => cc.id === c.id)
      )
  },
  watch: {
    keyboardWidth() {
      this.$store.graph.fitNode(this.node.id)
    }
  },
  created () {
    document.addEventListener('mouseup', this.onMouseupDocument)
  },
  async mounted () {
    await this.$nextTick() // avoid errors during minimize -> restore
    this.getZoom()
    const range = this.node.trigger.noteRange
    // eslint-disable-next-line vue/no-mutating-props
    this.node.trigger.noteRange = 0
    setTimeout(() => {
      this.$store.graph.fitNode(this.node.id) // FIX ports not aligning correctly
      // eslint-disable-next-line vue/no-mutating-props
      this.node.trigger.noteRange = range // FIX initial width v-bind not working properly on Gtk linux
    }, 0);
  },
  beforeUnmount () {
    document.removeEventListener('mouseup', this.onMouseupDocument)
  },
  methods: {
    getZoom () {
      let parent = this.$parent
      while (parent && !parent.getZoom) {
        parent = parent.$parent
      }
      this.zoom = parent?.getZoom() || 1
    },
    // returns number of whitekeys before a blackkey
    getBlackSpacing (i) {
      return [1,2,4,5,6][i % 5] + Math.floor(i / 5) * 7
    },
    onMouseupDocument () {
      this.keydown = null
    },
    hubProcess (bytes) {
      this.$store.app.hubProcess({
        ts: Date.now(),
        bytes,
        from: this.node.id,
        to: '*',
        fromPort: '*',
        toPort: '*'
      })
    },
    onMousedownKey (key) {
      this.keydown = key
      this.hubProcess([0x90 | this.node.trigger.channel, key, 127])
    },
    onMouseupKey (key) {
      this.keydown = null
      this.hubProcess([0x80 | this.node.trigger.channel, key, 0])
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
    },
    zeroSliderDefer(i) {
      setTimeout(() => {
        const slider = this.trigger.sliders[i]
        slider.value = 0
        slider.switchOn = false
        this.dispatchSlider(i)
      }, 0);
    },
    async dispatchSlider(i) {
      await this.$nextTick()
      const slider = this.triggerSliders[i]
      let value = this.trigger.sliders[i].value
      const channel = this.trigger.channel
      const msg1 = []
      const msg2 = []

      if (slider.mode === 'pitch') {
        msg1.push(0xE0 | channel)
        value += 8192
        const lsb = value & 0x7F
        const msb = (value >> 7) & 0x7F
        msg1.push(msb)
        msg1.push(lsb)
      } else if (slider.mode === 'channelAT') {
        msg1.push(0xD0 | channel)
        msg1.push(value)
      } else if (slider.mode === 'switch') {
        if (value >= 64 && !this.trigger.sliders[i].switchOn) {
          this.trigger.sliders[i].switchOn = true
          msg1.push(0xB0 | channel)
          msg1.push(slider.cc)
          msg1.push(127)
        } else if (value < 64 && this.trigger.sliders[i].switchOn) {
          this.trigger.sliders[i].switchOn = false
          msg1.push(0xB0 | channel)
          msg1.push(slider.cc)
          msg1.push(0)
        }
      } else if (slider.mode === 'HR') {
        let msb, lsb
        if (slider.min < 0) {
          value += 8192
        }
        lsb = value & 0x7F
        msb = (value >> 7) & 0x7F
        msg1.push(0xB0 | channel)
        msg1.push(slider.cc1)
        msg1.push(msb)
        msg2.push(0xB0 | channel)
        msg2.push(slider.cc2)
        msg2.push(lsb)
      } else {
        msg1.push(0xB0 | channel)
        msg1.push(slider.cc)
        msg1.push(value)
      }
      if (msg1.length) {
        this.hubProcess(msg1)
      }
      if (msg2.length) {
        this.hubProcess(msg2)
      }
    },
    onSliderDragStart () {
      let graphView = this.$parent
      while (graphView && !graphView.pan) {
        graphView = graphView.$parent
      }
      if (graphView) {
        graphView.ignoreClicks = true
      }
    },
    onSliderDragEnd (i) {
      let graphView = this.$parent
      while (graphView && !graphView.pan) {
        graphView = graphView.$parent
      }
      if (graphView) {
        graphView.ignoreClicks = false
      }
      const slider = this.trigger.sliders[i]
      if (slider.reset) {
        slider.value = 0
        this.dispatchSlider()
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
    <template v-if="triggerSliders.length" #header-bottom>
      <div class="sliders">
        <vue-slider
          v-for="slider,i in triggerSliders" :key="slider.id + i"
          v-model="trigger.sliders[i].value"
          :min="slider.min"
          :max="slider.max"
          :tooltip="'none'"
          :contained="true"
          :process="pos => slider.min < 0 ? [[50, pos[0]]] : [[0, pos[0]]]"
          :zoom="zoom"
          @change="dispatchSlider(i)"
          @dblclick="zeroSliderDefer(i)"
          @drag-start="onSliderDragStart"
          @drag-end="onSliderDragEnd(i)"
          @mousedown.stop
          @mousedown.capture="getZoom"
          @click.stop
        ></vue-slider>
      </div>
    </template>
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
:deep(.content) {
  background: none;
}
.sliders {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-left: -6px;
  margin-right: -12px;
  padding-bottom: 4px;
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