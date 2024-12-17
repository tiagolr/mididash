<script>
import { stripPrefix } from '../../utils';
import ReplacePopup from './ReplacePopup.vue'
import Checkbox from '../global/forms/Checkbox.vue';
import InspMapper from './InspMapper.vue';
import InspDelay from './InspDelay.vue';
import InspScript from './InspScript.vue';
import InspTrigger from './InspTrigger.vue'
export default {
  components: {
    ReplacePopup,
    Checkbox,
    InspMapper,
    InspDelay,
    InspScript,
    InspTrigger
  },
  data() {
    return {
      replacePopup: false,
    }
  },
  computed: {
    device: vm => vm.$store.graph
      .getNode(vm.$store.graph.selected),
    isInput: vm => vm.device?.class === 'input',
    isOutput: vm => vm.device?.class === 'output'
  },
  methods: {
    stripPrefix(id) {
      return stripPrefix(id)
    },
    onEditName () {
      this.$store.graph.setDeviceName(this.device.id, this.$refs.name.innerText)
      this.$refs.name.blur() // in case enter key was pressed
    },
    onEditOutputName (e, portId) {
      this.$store.graph.setOutputName(this.device.id, portId, e.target.innerText)
      e.target.blur()
    },
    removeDevice () {
      this.$store.graph.removeDevice(this.device.id)
    }
  }
}
</script>

<template>
  <div class="inspector cursor-default">
    <div v-if="!device" class="font-lighter">
      Select a node
    </div>
    <div v-else>
      <div class="flex">
        <div class="font-semi mb-1rem" style="text-transform: capitalize;">{{ device.class }}</div>
        <div class="flex-right" style="margin-top: -8px">
          <button class="button error " @click="removeDevice">
            Remove
          </button>
        </div>
      </div>
      <div v-if="device.class === 'input' || device.class === 'output'" class="field font-lighter mb-1rem">
        {{ stripPrefix(device.id) }}
      </div>
      <div v-if="device.disconnected" class="warn-disconnected mb-1rem">
        <div>This device is not connected. Reconnect or replace the device.</div>
        <div class="flex mt-05rem">
          <button class="button flex-1" @click="replacePopup = true">
            Replace device
          </button>
        </div>
      </div>
      <div class="font-lighter">Name</div>
      <div
        ref="name"
        class="field field-dark"
        contenteditable
        style="outline: none; cursor:text"
        @blur="onEditName"
        @keydown.enter.prevent.stop="onEditName"
      >
        {{ device.name }}
      </div>
      <div v-if="device.class === 'split'">
        <div class="font-lighter mt-1rem">
          Out ports
        </div>
        <div class="list compact panel mt-025rem">
          <div class="overflow">
            <div v-for="output in device.outPorts.filter(p => p.id !== '*')" :key="output.id" class="list-item">
              <checkbox :checked="!output.hidden" @click="$store.graph.toggleOutport(device.id, output.id)">
              </checkbox>
              <div
                contenteditable
                @blur="(e) => onEditOutputName(e, output.id)"
                @keydown.enter.prevent.stop="e => onEditOutputName(e, output.id)"
              >
                {{ output.name }}
              </div>
            </div>
          </div>
        </div>
      </div>
      <div v-if="device.class === 'map'">
        <insp-mapper :device="device">
        </insp-mapper>
      </div>
      <div v-if="device.class === 'delay'">
        <insp-delay :device="device">
        </insp-delay>
      </div>
      <div v-if="device.class === 'script'">
        <insp-script :device="device">
        </insp-script>
      </div>
      <div v-if="device.class === 'trigger'">
        <insp-trigger :device="device">
        </insp-trigger>
      </div>

      <replace-popup v-if="replacePopup" :device="device" @close="replacePopup = false">
      </replace-popup>
    </div>
  </div>
</template>


<style scoped>
.inspector {
  display: flex;
  flex-direction: column;
  overflow: auto;
  padding-top: 1rem;
  padding-bottom: 1rem;
}
.inspector::-webkit-scrollbar {
  display:none;
}
:deep(.field) {
  margin-top: 0.25rem;
}
.field-dark {
  padding: 0.5rem;
  background: var(--foreground);
  border-radius: var(--panel-radius);
}
.warn-disconnected {
  /* padding: 0.5rem; */
  border-radius: var(--panel-radius);
  /* background: var(--warning); */
  color: var(--warning-content)
}
:deep(.panel) {
  background: var(--foreground);
  border-radius: var(--panel-radius);
  flex-shrink: 0;
  padding: 0.25rem;
}
:deep(.list) {
  display: flex;
  flex-direction: column;
  max-height: 180px;
}
:deep(.list-item) {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0.4rem;
  flex-shrink: 0;
  border-radius: var(--panel-radius);
}
:deep(.list.compact .list-item) {
 padding: 0.2rem 0.4rem;
}
</style>
