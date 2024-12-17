<script>
import { stripPrefix } from '../../utils';
import Popup from '../global/Popup.vue'
import ListSelect from '../global/forms/ListSelect.vue'
export default {
    components: {
      Popup,
      ListSelect,
    },
    props: {
      device: Object
    },
    emits: [
      'close'
    ],
    data() {
      return {
        stripPrefix,
        name: '',
        selected: null,
      }
    },
    computed: {
      io: vm => vm.device.class === 'input'
        ? vm.$store.app.midiPorts.inputs
        : vm.$store.app.midiPorts.outputs,
      options: vm => vm.io
        .filter(io => !vm.$store.graph.nodes
          .some(n => n.class === vm.device.class && stripPrefix(n.id) === io))
        .map(io => ({ id: io, label: io }))
    },
    watch: {
      selected (id) {
        this.name = id.split(':')[0]
      }
    },
    methods: {
      onConfirm () {
        this.$store.graph.replaceIO(this.device, this.selected, this.name)
      }
    }
}
</script>

<template>
  <popup @close="$emit('close')">
    <div class="replace-popup">
      <div class="title">
        <div>Replace</div>
        <div class="capitalize">{{ device.class }}</div>
      </div>
      <div class="mb-1rem">{{ stripPrefix(device.id) }}</div>
      <list-select
        hide-checkboxes
        close-on-select
        :selected="selected ? [selected] : []"
        :placeholder="selected || 'Select device'"
        :no-options-text="'No device available'"
        :options="options"
        @selected="id => selected = id "
      ></list-select>
      <div class="font-lighter mt-4 font-12">
        Device must not be on the viewport
      </div>
      <div class="mt-1rem mb-025rem font-lighter">
        Name
      </div>
      <input v-model="name" type="text" class="input">
      <div class="mt-2rem flex gap-05rem">
        <button class="button ghost big flex-1" @click="$emit('close')">
          Cancel
        </button>
        <button class="button primary big flex-1" :disabled="!selected" @click="onConfirm">
          Confirm
        </button>
      </div>
    </div>
  </popup>
</template>


<style scoped>
.replace-popup {
  width: 325px;
}
.title {
  display: flex;
  /* justify-content: center; */
  gap: 5px;
  font-size: 1.1rem;
  font-weight: 600;
  text-align: center;
  margin-bottom: 2rem;
}
</style>