<script>
import { MAPPER_MSGS } from '../../globals';
import NumberInput from '../global/forms/NumberInput.vue';
import Checkbox from '../global/forms/Checkbox.vue'
export default {
  components: {
    NumberInput,
    Checkbox
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
          data2Max: -1,
          pullData1: false,
          pullData2: false
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
      this.cols.output.pullData1 = this.device.rule.pullData1
      this.cols.output.pullData2 = this.device.rule.pullData2
    },
    saveRule () {
      const rule = {
        inChannel: this.cols.input.channel,
        inType: this.cols.input.type,
        inData1Min: this.cols.input.data1Min,
        inData1Max: this.cols.input.data1Max,
        inData2Min: this.cols.input.data2Min,
        inData2Max: this.cols.input.data2Max,
        outChannel: this.cols.output.channel,
        outType: this.cols.output.type,
        outData1Min: this.cols.output.data1Min,
        outData1Max: this.cols.output.data1Max,
        outData2Min: this.cols.output.data2Min,
        outData2Max: this.cols.output.data2Max,
        pullData1: this.cols.output.pullData1,
        pullData2: this.cols.output.pullData2
      }
      this.$store.graph.setDeviceData(this.device.id, 'rule', rule)
    }
  }
}
</script>

<template>
  <div class="mt-1rem gap-8 flex-column gap-8">
    <div v-for="(io, key) in cols" :key="key" class="rule flex-column flex-1">
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
      <div class="flex-column gap-4">
        <div class="flex-center">
          <div class="label">Data1</div>
          <number-input v-model="io.data1Min" :min="-1" :max="127" placeholder="Min" style="width: 60px" show-placeholder-on-min @change="saveRule">
          </number-input>
          <number-input v-model="io.data1Max" :min="-1" :max="127" placeholder="Max" style="width: 60px" show-placeholder-on-min @change="saveRule">
          </number-input>
          <div v-if="key === 'output'" class="flex-center gap-4" style="margin-left: 12px" title="Use data 2 input instead of data 1">
            <checkbox :checked="io.pullData1" @click="() => { io.pullData1 = !io.pullData1; saveRule() }">
            </checkbox>
            <div class="font-lighter">
              Pull 2
            </div>
          </div>
        </div>
        <div class="flex-center" :class="(io.type === 0x0C || io.type === 0x0D) && 'opacity-05'">
          <div class="label">Data2</div>
          <number-input v-model="io.data2Min" :min="-1" :max="127" placeholder="Min" style="width: 60px" show-placeholder-on-min @change="saveRule">
          </number-input>
          <number-input v-model="io.data2Max" :min="-1" :max="127" placeholder="Max" style="width: 60px" show-placeholder-on-min @change="saveRule">
          </number-input>
          <div v-if="key === 'output'" class="flex-center gap-4" style="margin-left: 12px" title="Use data 1 input instead of data 2">
            <checkbox :checked="io.pullData2" @click="() => { io.pullData2 = !io.pullData2; saveRule() }">
            </checkbox>
            <div class="font-lighter">
              Pull 1
            </div>
          </div>
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