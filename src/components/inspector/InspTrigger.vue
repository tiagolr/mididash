<script>
import { TRIGGER_CC_HR, TRIGGER_CC_RAW } from '../../midi-data'
import VueSlider from "vue-3-slider-component"
import NumberInput from '../global/forms/NumberInput.vue'
import Checkbox from '../global/forms/Checkbox.vue'
import SwitchButton from '../global/forms/SwitchButton.vue'
import IClose from '../../assets/close.svg'
export default {
  components: {
    VueSlider,
    NumberInput,
    Checkbox,
    SwitchButton,
    IClose,
  },
  props: {
    device: Object
  },
  data() {
    return {
      TRIGGER_CC_RAW,
      TRIGGER_CC_HR,
      trigger: this.device.trigger
    }
  },
  computed: {
    triggerSliders: vm => vm.trigger.sliders
      .map(c => c.raw
        ? TRIGGER_CC_RAW.find(cc => cc.id === c.id)
        : TRIGGER_CC_HR.find(cc => cc.id === c.id)
      )
  },
  watch: {
    device (d) {
      this.trigger = d.trigger
    }
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
    hubProcess (bytes) {
      this.$store.app.hubProcess({
        ts: Date.now(),
        bytes,
        from: this.device.id,
        to: '*',
        fromPort: '*',
        toPort: '*'
      })
    },
    sendBytes () {
      const bytes = this.trigger.bytes
        .split(/\s+/)
        .map(s => parseInt(s))
        .filter(i => !isNaN(i))
        .map(i => i % 256)
      this.hubProcess(bytes)
    },
    zeroSlider(i) {
      const slider = this.trigger.sliders[i]
      slider.value = 0
      slider.switchOn = false
    },
    zeroSliderDefer(i) {
      setTimeout(() => {
        this.zeroSlider(i)
        this.dispatchSlider(i)
      }, 0);
    },
    resetSlider(evt, i) {
      const id = evt.target.value
      this.zeroSlider(i)
      // FIX - force slider id change, capture event interfers with v-model
      // and its necessary to reset the value before slider id change otherwise
      // results in console errors because the slider value may be outside min max bounds
      this.trigger.sliders[i].id = id
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
    toggleSliderRaw(i) {
      this.zeroSlider(i)
      const slider = this.trigger.sliders[i]
      slider.raw = !slider.raw
    },
    toggleSliderVisible(i) {
      const slider = this.trigger.sliders[i]
      slider.visible = !slider.visible
    },
    toggleSliderReset(i) {
      this.zeroSlider(i)
      const slider = this.trigger.sliders[i]
      slider.reset = !slider.reset
    },
    toggleSwitchValue(i) {
      const slider = this.trigger.sliders[i]
      slider.value = slider.value > 63 ? 0 : 127
      this.dispatchSlider(i)
    },
    onSliderDragEnd (i) {
      const slider = this.trigger.sliders[i]
      if (slider.reset) {
        slider.value = 0
        this.dispatchSlider(i)
      }
    },
    removeSlider (i) {
      this.trigger.sliders.splice(i, 1)
    },
    addSlider () {
      this.trigger.sliders.push({
        id: 'pitch',
        value: 0,
        raw: false,
        visible: false,
        reset: false,
        switchOn: false // field to toggle switch on/off when slider crosses middle
      })
    }
  }
}
</script>

<template>
  <select v-model="trigger.channel" class="select mt-1rem">
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
  <div class="mt-1rem mb-025rem flex-center">
    <div class="font-lighter">
      Sliders
    </div>
    <div class="flex-right cursor-pointer" @click="addSlider">
      + Add
    </div>
  </div>
  <div v-for="slider,i in triggerSliders" :key="slider.id+i" class="slider">
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
      <div class="flex-right">
        <i-close class="icon" @click="removeSlider(i)">
        </i-close>
      </div>
    </div>
    <div class="flex-center gap-4">
      <select v-model="trigger.sliders[i].id" class="select" @change.capture="evt => resetSlider(evt, i)">
        <option v-for="cc in (trigger.sliders[i].raw ? TRIGGER_CC_RAW : TRIGGER_CC_HR)" :key="cc.id" :value="cc.id">
          {{ cc.label }}
        </option>
      </select>
      <switch-button
        v-if="slider.mode === 'switch'"
        small
        color
        :no-text="'Off'"
        :yes-text="'On'"
        :checked="trigger.sliders[i].value > 63"
        @click="toggleSwitchValue(i)"
      ></switch-button>
      <number-input
        v-else
        v-model="trigger.sliders[i].value"
        :min="slider.min"
        :max="slider.max"
        @change="dispatchSlider(i)"
      ></number-input>
    </div>
    <div>
      <vue-slider
        v-model="trigger.sliders[i].value"
        class="mt-05rem"
        :min="slider.min"
        :max="slider.max"
        :tooltip="'none'"
        :contained="true"
        :process="pos => slider.min === -slider.max ? [[50, pos[0]]] : [[0, pos[0]]]"
        @change="dispatchSlider(i)"
        @drag-end="onSliderDragEnd(i)"
        @dblclick="zeroSliderDefer(i)"
      ></vue-slider>
    </div>
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
.icon {
  width: 17px;
  height: 17px;
  cursor: pointer;
}
:deep(.icon path) {
  fill: var(--text);
}
</style>