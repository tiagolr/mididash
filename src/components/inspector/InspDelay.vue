<script>
import { millisToSecondsStr } from '../../utils';
import NumberInput from '../global/forms/NumberInput.vue';
export default {
  components: {
    NumberInput
  },
  props: {
    device: Object
  },
  data() {
    return {
      delay: this.device.delay
    }
  },
  watch: {
    device () {
      this.delay = this.device.delay
    }
  },
  methods: {
    update() {
      this.$store.graph.setDeviceData(this.device.id, 'delay', this.delay)
      this.$store.graph.setDeviceProperty(this.device.id, 'delay', this.delay)
      let name = this.device.name
      const regx = /\d+(\.\d+)?(ms|s)/
      if (regx.test(name)) {
        name = name
          .replace(regx, '')
          .concat(millisToSecondsStr(this.delay))
      }
      this.$store.graph.setDeviceName(this.device.id, name)
    }
  }
}
</script>

<template>
  <div class="font-lighter mt-1rem mb-025rem">
    Time (ms)
  </div>
  <number-input v-model="delay" :min="0" :max="60000" style="max-width: 65px" @change="update">
  </number-input>
</template>


<style scoped>
</style>