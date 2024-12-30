<script>
import { TRIGGER_CCHR as TRIGGER_CC_RAW } from '../../midi-data'
import VueSlider from "vue-3-slider-component"
import NumberInput from '../global/forms/NumberInput.vue'
import Checkbox from '../global/forms/Checkbox.vue'
export default {
  components: {
    VueSlider,
    NumberInput,
    Checkbox
  },
  props: {
    device: Object
  },
  data() {
    return {
      TRIGGER_CC_RAW,
      trigger: this.device.trigger
    }
  },
  computed: {
    triggerSliders: vm => vm.trigger.sliders
      .map(c => TRIGGER_CC_RAW.find(cc => cc.id === c.id))
  },
  methods: {
    saveTrigger () {
      this.$store.graph.setDeviceProperty(this.device.id, 'trigger', this.trigger)
    },
    saveBytes () {
      this.trigger.bytes = this.$refs.bytes.innerText
      this.saveTrigger()
    },
    onEnterBytes (evt) {
      if (evt.ctrlKey || evt.metaKey) {
        this.saveBytes()
        this.sendBytes()
      }
    },
    sendBytes () {
      const bytes = this.trigger.bytes
        .split(/\s+/)
        .map(s => parseInt(s))
        .filter(i => !isNaN(i))
        .map(i => i % 256)

      this.$store.app.hubProcess({
        ts: Date.now(),
        bytes,
        from: this.device.id,
        to: '*',
        fromPort: '*',
        toPort: '*'
      })
    },
    resetSlider(i) {
      this.trigger.sliders[i].value = 0
      this.saveTrigger()
    },
    dispatchSlider(i) {
      const val = this.trigger.sliders[i].value
      console.log(val)
    },
    toggleSliderRaw(i) {
    },
    toggleSliderVisible(i) {
    },
    toggleSliderReset(i) {
    }
  }
}
</script>

<template>
  <select v-model="trigger.channel" class="select mt-1rem" @change="saveTrigger">
    <option v-for="i in 16" :key="i" :value="i - 1">
      Channel {{ i }}
    </option>
  </select>
  <div class="mt-1rem font-lighter mb-025rem">
    Octaves
  </div>
  <div class="flex-center gap-8">
    <div>Start</div>
    <select v-model="trigger.noteStart" class="select">
      <option :value="0">C-1</option>
      <option :value="1">C0</option>
      <option :value="2">C1</option>
      <option :value="3">C2</option>
      <option :value="4">C3</option>
      <option :value="5">C4</option>
      <option :value="6">C5</option>
      <option :value="7">C6</option>
      <option :value="8">C7</option>
      <option :value="9">C8</option>
      <option :value="10">C9</option>
    </select>
    <div style="margin-left: 4px">Range</div>
    <select v-model="trigger.noteRange" class="select">
      <option :value="1">1</option>
      <option :value="2">2</option>
      <option :value="3">3</option>
      <option :value="4">4</option>
      <option :value="5">5</option>
      <option :value="6">6</option>
      <option :value="7">7</option>
      <option :value="8">8</option>
    </select>
  </div>
  <div class="mt-1rem font-lighter mb-025rem">
    Sliders
  </div>
  <div v-for="slider,i in triggerSliders" :key="slider.id" class="slider">
    <div class="flex-center gap-8 mb-05rem">
      <div class="flex-center gap-4">
        <checkbox :checked="trigger.sliders[i].raw" @click="toggleSliderRaw(i)">
        </checkbox>
        Raw
      </div>
      <div class="flex-center gap-4">
        <checkbox :checked="trigger.sliders[i].visible" @click="toggleSliderVisible(i)">
        </checkbox>
        Visible
      </div>
      <div class="flex-center gap-4">
        <checkbox :checked="trigger.sliders[i].reset" @click="toggleSliderReset(i)">
        </checkbox>
        Reset
      </div>
    </div>
    <div class="flex-center gap-4">
      <select v-model="trigger.sliders[i].id" class="select" @change="resetSlider(i)">
        <option v-for="cc in TRIGGER_CC_RAW" :key="cc.id" :value="cc.id">
          {{ cc.label }}
        </option>
      </select>
      <number-input v-model="trigger.sliders[i].value" :min="slider.min" :max="slider.max">
      </number-input>
    </div>
    <vue-slider
      v-model="trigger.sliders[i].value"
      class="mt-05rem"
      :min="slider.min"
      :max="slider.max"
      :tooltip="'none'"
      :contained="true"
      :process="pos => slider.min === -slider.max ? [[50, pos[0]]] : [[0, pos[0]]]"
      @change="dispatchSlider(i)"
    ></vue-slider>
  </div>
  <div class="mt-1rem font-lighter mb-025rem">
    Bytes
  </div>
  <div class="panel">
    <div
      ref="bytes"
      class="overflow bytes"
      contenteditable
      @blur="saveBytes"
      @keydown.enter="onEnterBytes"
    >
      {{ device.trigger.bytes }}
    </div>
  </div>
  <button class="button mt-05rem" @click="sendBytes">
    Send
  </button>
</template>


<style scoped>
.panel {
  padding: 0.5rem !important;
  max-height: 180px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
.bytes {
  outline: none;
  cursor: text;
}
.slider {
  padding: 0.5rem;
  border: 1px solid var(--border);
  border-radius: var(--panel-radius);
}
</style>