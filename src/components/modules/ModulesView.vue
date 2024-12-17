<script>
import { stripPrefix } from '../../utils';
import IConfig from '../../assets/wrench.svg'
import ScriptTemplatesPopup from './ScriptTemplatesPopup.vue';
import IInput from '../../assets/input.svg'
import IOutput from '../../assets/output.svg'
import IVirtual from '../../assets/virtual.svg'
import IMap from '../../assets/map.svg'
import ISplit from '../../assets/split.svg'
import IDelay from '../../assets/delay.svg'
import IMonitor from '../../assets/monitor.svg'
import INote from '../../assets/note.svg'
import IScript from '../../assets/script.svg'
import ITrigger from '../../assets/piano.svg'

export default {
  components: {
    ScriptTemplatesPopup,
    IConfig,
    IInput,
    IOutput,
    IVirtual,
    IMap,
    ISplit,
    IDelay,
    IMonitor,
    INote,
    IScript,
    ITrigger,
  },
  data() {
    return {
      scriptTemplatesPopup: false
    }
  },
  computed: {
    inputs: vm => [...vm.$store.app.midiPorts.inputs].reverse(),
    addedInputs: vm => vm.$store.graph.nodes
      .filter(n => n.class === 'input')
      .map(n => ({ id: n.id, strip: stripPrefix(n.id) })),
    selectedInputs: vm => vm.addedInputs
      .filter(i => i.id === vm.$store.graph.selected)
      .map(o => o.strip),
    outputs: vm => [...vm.$store.app.midiPorts.outputs].reverse(),
    addedOutputs: vm => vm.$store.graph.nodes
      .filter(n => n.class === 'output')
      .map(n => ({ id: n.id, strip: stripPrefix(n.id) })),
    selectedOutputs: vm => vm.addedOutputs
      .filter(o => o.id === vm.$store.graph.selected)
      .map(o => o.strip),
  },
  methods: {
    onDragstart (event, device) {
      event.dataTransfer.setData('text/plain', 'draggedElement'); // FIX drag and drop https://github.com/tauri-apps/tauri/issues/6695
      this.$store.graph.dragdropDevice = device // another fix, keep the device on store, its currently not working to pass data on events
    },
    onDragend () {
      this.$store.graph.dragdropDevice = null
    },
  }
}
</script>

<template>
  <div class="modules cursor-default">
    <div class="font-lighter mt-1rem">
      Inputs
    </div>
    <div class="list panel mt-025rem">
      <div class="overflow">
        <div
          v-for="input in inputs" :key="input"
          class="input list-item flex gap-8"
          :title="input"
          :class="{
            disabled: addedInputs.find(i => i.strip === input),
          }"
          :draggable="!addedInputs.find(i => i.strip === input)"
          @dragstart="e => onDragstart(e, { class: 'input', id: input })"
          @dragend="onDragend"
        >
          <i-input class="icon" :class="addedInputs.find(i => i.strip === input) && 'disabled'">
          </i-input>
          <div class="text-ellipsis">{{ input }}</div>
        </div>
      </div>
    </div>
    <div class="font-lighter mt-1rem">
      Outputs
    </div>
    <div class="list panel mt-025rem">
      <div class="overflow">
        <div
          v-for="output in outputs" :key="output"
          class="output list-item text-ellipsis flex gap-8"
          :title="output"
          :class="{
            disabled: addedOutputs.find(o => o.strip === output),
          }"
          :draggable="!addedOutputs.find(o => o.strip === output)"
          @dragstart="e => onDragstart(e, { class: 'output', id: output })"
          @dragend="onDragend"
        >
          <i-output class="icon" :class="addedOutputs.find(o => o.strip === output) && 'disabled'">
          </i-output>
          <div class="text-ellipsis">{{ output }}</div>
        </div>
      </div>
    </div>
    <div v-if="$store.app.os !== 'windows'">
      <div class="font-lighter mt-1rem">
        Virtual
      </div>
      <div class="list panel mt-025rem">
        <div
          class="list-item flex gap-8" :draggable="true"
          @dragstart="e => onDragstart(e, { class: 'virtual' })"
          @dragend="onDragend"
        >
          <i-virtual class="icon">
          </i-virtual>
          <div class="text-ellipsis">Virtual cable</div>
        </div>
      </div>
    </div>
    <div class="font-lighter mt-1rem">
      Base
    </div>
    <div class="list panel mt-025rem">
      <div class="overflow">
        <div
          class="list-item flex gap-8" :draggable="true"
          @dragstart="e => onDragstart(e, { class: 'split' })"
          @dragend="onDragend"
        >
          <i-split class="icon">
          </i-split>
          <div>Split</div>
        </div>
        <div
          class="list-item flex gap-8" :draggable="true"
          @dragstart="e => onDragstart(e, { class: 'map' })"
          @dragend="onDragend"
        >
          <i-map class="icon">
          </i-map>
          <div>Map</div>
        </div>
        <div
          class="list-item flex gap-8" :draggable="true"
          @dragstart="e => onDragstart(e, { class: 'delay' })"
          @dragend="onDragend"
        >
          <i-delay class="icon">
          </i-delay>
          <div>Delay</div>
        </div>
        <div
          class="list-item flex gap-8" :draggable="true"
          @dragstart="e => onDragstart(e, { class: 'monitor' })"
          @dragend="onDragend"
        >
          <i-monitor class="icon">
          </i-monitor>
          <div>Monitor</div>
        </div>
        <div
          class="list-item flex gap-8" :draggable="true"
          @dragstart="e => onDragstart(e, { class: 'trigger' })"
          @dragend="onDragend"
        >
          <i-trigger class="icon">
          </i-trigger>
          <div>Trigger</div>
        </div>
        <div
          class="list-item flex gap-8" :draggable="true"
          @dragstart="e => onDragstart(e, { class: 'note' })"
          @dragend="onDragend"
        >
          <i-note class="icon">
          </i-note>
          <div>Note</div>
        </div>
      </div>
    </div>
    <div class="flex mt-1rem">
      <div class="font-lighter">
        Scripts
      </div>
      <i-config class="flex-right icon-config" @click="scriptTemplatesPopup = true">
      </i-config>
    </div>
    <div v-if="$store.app.settings.scriptTemplates.length" class="list panel mt-025rem">
      <div class="overflow">
        <div
          v-for="template,i in $store.app.settings.scriptTemplates" :key="i"
          class="list-item flex gap-8" :draggable="true"
          @dragstart="e => onDragstart(e, { class: 'script', script: template.script, outPorts: template.outPorts, name: template.name })"
          @dragend="onDragend"
        >
          <i-script class="icon">
          </i-script>
          <div>{{ template.name }}</div>
        </div>
      </div>
    </div>

    <script-templates-popup
      v-if="scriptTemplatesPopup"
      @close="scriptTemplatesPopup = false"
    ></script-templates-popup>
  </div>
</template>


<style scoped>
.modules {
  overflow: auto;
  padding-bottom: 1rem;
}
.modules::-webkit-scrollbar {
  display:none;
}
.panel {
  background: var(--foreground);
  border-radius: var(--panel-radius);
  flex-shrink: 0;
  padding: 0.25rem;
}
.list {
  display: flex;
  flex-direction: column;
  max-height: 200px;
}
.list-item {
  padding: 0 0.4rem;
  height: 32px;
  display: flex;
  align-items: center;
  flex-shrink: 0;
  border-radius: var(--panel-radius);
}
.list-item:hover {
  background: var(--node-color);
  cursor: move;
}
.list-item.disabled {
  color: var(--copy-lighter);
}
.selected {
  box-shadow: inset 0px 0px 0px 1px var(--success-content);
}
.disabled:hover {
  background: none;
  cursor: default;
}
.icon-config {
  width: 17px;
  height: 17px;
  cursor: pointer;
}
.icon {
  width: 19px;
  height: 19px;
  /* margin-top: -2px; */
  /* margin-bottom: -20px; */
  flex-shrink: 0;
}
.icon.disabled {
  opacity: 0.5;
}
</style>