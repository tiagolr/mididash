<script>
import { MAPPER_MSGS } from '../../globals';
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
      MAPPER_MSGS,
      test: -1,
      cols: {
        input: {
          channel: -1,
          type: -1,
          data1Min: -1,
          data1Max: -1,
          data2Min: -1,
          data2Max: -1,
        },
        output: {
          channel: -1,
          type: -1,
          data1Min: -1,
          data1Max: -1,
          data2Min: -1,
          data2Max: -1
        }
      }
    }
  },
  watch: {
    'device.rule': 'resetForm',
  },
  beforeMount () {
    this.resetForm()
  },
  methods: {
    resetForm () {
      this.cols.input.channel = this.device.rule.inChannel
      this.cols.input.type = this.device.rule.inType
      this.cols.input.data1Min = this.device.rule.inData1Min
      this.cols.input.data1Max = this.device.rule.inData1Max
      this.cols.input.data2Min = this.device.rule.inData2Min
      this.cols.input.data2Max = this.device.rule.inData2Max
      this.cols.output.channel = this.device.rule.outChannel
      this.cols.output.type = this.device.rule.outType
      this.cols.output.data1Min = this.device.rule.outData1Min
      this.cols.output.data1Max = this.device.rule.outData1Max
      this.cols.output.data2Min = this.device.rule.outData2Min
      this.cols.output.data2Max = this.device.rule.outData2Max
    },
    saveRule () {
      const rule = {
        in_channel: this.cols.input.channel,
        in_type: this.cols.input.type,
        in_data1Min: this.cols.input.data1Min,
        in_data1Max: this.cols.input.data1Max,
        in_data2Min: this.cols.input.data2Min,
        in_data2Max: this.cols.input.data2Max,
        out_channel: this.cols.output.channel,
        out_type: this.cols.output.type,
        out_data1Min: this.cols.output.data1Min,
        out_data1Max: this.cols.output.data1Max,
        out_data2Min: this.cols.output.data2Min,
        out_data2Max: this.cols.output.data2Max,
      }
      this.$store.graph.setDeviceData(this.device.id, 'rule', rule)
    }
  }
}
</script>

<template>
  <div class="mt-1rem gap-8 flex-column gap-8">
    <div v-for="(io, key) in cols" :key="key" class="rule flex-wrap flex-1">
      <div style="font-weight: 600;text-transform: uppercase;width: 100%;" class="uppercase">
        {{ key }}
      </div>
      <div class="flex gap-4">
        <div>
          <div class="label">Channel</div>
          <select v-model="io.channel" class="select" @change="saveRule">
            <option :value="-1">{{ key === 'input' ? 'Any' : 'Copy' }}</option>
            <option v-for="channel in [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]" :key="channel" :value="channel">
              {{ channel+1 }}
            </option>
          </select>
        </div>
        <div>
          <div class="label">Message</div>
          <select v-model="io.type" class="select" @change="saveRule">
            <option :value="-1">{{ key === 'input' ? 'Any' : 'Copy' }}</option>
            <option v-for="msg in MAPPER_MSGS" :key="msg.name" :value="msg.value">{{ msg.name }}</option>
          </select>
        </div>
      </div>
      <div class="flex flex-wrap gap-4">
        <div class="flex-center">
          <div class="label">Data1</div>
          <number-input v-model="io.data1Min" :min="-1" :max="127" placeholder="Min" style="width: 60px" show-placeholder-on-min @change="saveRule">
          </number-input>
          <number-input v-model="io.data1Max" :min="-1" :max="127" placeholder="Max" style="width: 60px" show-placeholder-on-min @change="saveRule">
          </number-input>
        </div>
        <div class="flex-center">
          <div class="label">Data2</div>
          <number-input v-model="io.data2Min" :min="-1" :max="127" placeholder="Min" style="width: 60px" show-placeholder-on-min @change="saveRule">
          </number-input>
          <number-input v-model="io.data2Max" :min="-1" :max="127" placeholder="Max" style="width: 60px" show-placeholder-on-min @change="saveRule">
          </number-input>
        </div>
      </div>
    </div>
  </div>
</template>


<style scoped>
.rule > div {
  margin-bottom: 8px;
}
.rule > div > div:first-child {
  color: var(--text-lighter);
  margin-bottom: 2px;
}
.rule-input:first-child {
  margin-right: 8px;
}
.label {
  color: var(--text-lighter);
  margin-bottom: 2px;
}
</style>