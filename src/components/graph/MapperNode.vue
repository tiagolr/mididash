<script>
import { MAPPER_MSGS } from '../../globals';
import DeviceNode from './DeviceNode.vue';
import IMinus from '../../assets/minus.svg'
import IPlus from '../../assets/plus.svg'
export default {
  components: {
    DeviceNode,
    IMinus,
    IPlus,
  },
  props: {
    node: Object,
    dragConnector: Object
  },
  emits: [
    'mousedown-port',
    'do-connect'
  ],
  methods: {
    dataStr(num, placeholder) {
      return String(num === -1 ? placeholder : num).padStart(3, ' ')
    },
    ruleMessageType (msg, isInput) {
      return msg === -1 && isInput ? 'Any'
        : msg === -1 && !isInput ? 'Copy'
        : MAPPER_MSGS.find(m => m.value === msg)?.name || 'Unknown'
    },
    toggleMinimized () {
      this.$store.graph.setDeviceProperty(this.node.id, 'minimized', !this.node.minimized)
      this.$store.graph.fitNode(this.node.id)
    }
  }
}
</script>

<template>
  <device-node
    :inline="node.minimized"
    :device="node"
    :drag-connector="dragConnector"
    class="mapper-node"
    @mousedown-port="args => $emit('mousedown-port', args)"
    @mouseup-port="args => $emit('do-connect', args)"
  >
    <template #header>
      <i-plus v-if="node.minimized" class="icon flex-right" @click.stop="toggleMinimized">
      </i-plus>
      <i-minus v-else class="icon flex-right" @click.stop="toggleMinimized">
      </i-minus>
    </template>
    <div v-if="!node.minimized" class="content">
      <table>
        <tbody>
          <tr>
            <th></th>
            <th>Chn</th>
            <th>Msg</th>
            <th>Data1</th>
            <th>Data2</th>
          </tr>
          <tr>
            <td class="font-lighter text-right">IN</td>
            <td>{{ node.rule.inChannel === -1 ? 'Any' : node.rule.inChannel + 1 }}</td>
            <td>{{ ruleMessageType(node.rule.inType, true) }}</td>
            <td><pre>{{ dataStr(node.rule.inData1Min, 'Min') }} {{ dataStr(node.rule.inData1Max, 'Max') }}</pre></td>
            <td><pre>{{ dataStr(node.rule.inData2Min, 'Min') }} {{ dataStr(node.rule.inData2Max, 'Max') }}</pre></td>
          </tr>
          <tr>
            <td class="font-lighter text-right">OUT</td>
            <td>{{ node.rule.outChannel === -1 ? 'Copy' : node.rule.outChannel + 1 }}</td>
            <td>{{ ruleMessageType(node.rule.outType) }}</td>
            <td><pre>{{ dataStr(node.rule.outData1Min, 'Min') }} {{ dataStr(node.rule.outData1Max, 'Max') }}</pre></td>
            <td><pre>{{ dataStr(node.rule.outData2Min, 'Min') }} {{ dataStr(node.rule.outData2Max, 'Max') }}</pre></td>
          </tr>
        </tbody>
      </table>
    </div>
  </device-node>
</template>


<style scoped>
table {
  border-collapse: collapse;
  font-size: 13px;
}
table th {
  font-weight: normal;
  color: var(--copy-lighter);
  text-align: left;
}
table td, table th {
  padding: 2px 4px;
}
.mapper-node {
  border-radius: none;
}
.icon {
  width: 18px;
  height: 18px;
  cursor: pointer;
}
.d-node {
  background: none;
}
.content {
  /* background: #333c; */
  display: flex;
  flex-direction: column;
  padding: 8px;
  overflow: auto;
}
</style>