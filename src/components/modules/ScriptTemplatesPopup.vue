<script>
import Popup from '../global/Popup.vue'
import IArrow from '../../assets/down-arrow.svg'
import IClose from '../../assets/close.svg'
import { DEFAULT_SCRIPT_TEMPLATES } from '../../globals';
export default {
  components: {
    Popup,
    IArrow,
    IClose
  },
  emits: [
    'close'
  ],
  data() {
    return {
      templates: JSON.parse(JSON.stringify(this.$store.app.settings.scriptTemplates))
    }
  },
  methods: {
    moveUp (temp) {
      const index = this.templates.indexOf(temp)
      if (index > 0) {
        console.log('UP')
        this.templates.splice(index, 1)
        this.templates.splice(index - 1, 0, temp)
      }
    },
    moveDown (temp) {
      const index = this.templates.indexOf(temp)
      if (index < this.templates.length) {
        this.templates.splice(index, 1)
        this.templates.splice(index + 1, 0, temp)
      }
    },
    remove (temp) {
      const index = this.templates.indexOf(temp)
      this.templates.splice(index, 1)
    },
    restoreDefaults () {
      const defaults = JSON.parse(JSON.stringify(DEFAULT_SCRIPT_TEMPLATES))
      defaults.forEach(d => {
        if (!this.templates.find(dd => dd.name === d.name)) {
          this.templates.push(d)
        }
      })
    },
    async save () {
      await this.$store.app.setScriptTemplates(this.templates)
      this.$emit('close')
    },
    blur (evt) {
      evt.target.blur()
    },
    onBlur (evt, temp) {
      temp.name = evt.target.innerText.trim()
      console.log(temp, temp.name)
    }
  }
}
</script>

<template>
  <popup @close="$emit('close')">
    <div class="templates-popup">
      <div class="font-l mb-1rem font-bold">
        Script templates
      </div>
      <div v-if="templates.length" class="templates panel list">
        <div class="overflow">
          <div v-for="temp,i in templates" :key="i" class="template list-item">
            <div contenteditable="true" class="flex-1" @keydown.enter.stop.prevent="blur" @blur="evt => onBlur(evt, temp)">
              {{ temp.name }}
            </div>
            <i-arrow class="icon i-arrow" @click="moveDown(temp)" />
            <i-arrow class="icon i-arrow i-arrow-invert" @click="moveUp(temp)" />
            <i-close class="icon icon-close" @click="remove(temp)" />
          </div>
        </div>
      </div>
      <div class="mt-05rem font-lighter cursor-pointer btn-restore" @click="restoreDefaults">
        Restore defaults
      </div>
    </div>
    <div class="mt-15rem flex gap-05rem">
      <button class="button ghost flex-1" @click="$emit('close')">
        Cancel
      </button>
      <button class="button primary flex-1" @click="save">
        Save
      </button>
    </div>
  </popup>
</template>


<style scoped>
.templates-popup {
  width: 300px;
  max-width: 80vw;
}
.panel {
  background: var(--input);
  border-radius: var(--panel-radius);
  flex-shrink: 0;
  padding: 0.25rem;
}
.list {
  display: flex;
  flex-direction: column;
  max-height: 180px;
}
.list-item {
  padding: 0.4rem;
  flex-shrink: 0;
  border-radius: var(--panel-radius);
  display: flex;
  align-items: center;
}
.template * {
  outline: none
}
.template:hover .icon {
  display: block;
}
.icon {
  width: 17px;
  height: 17px;
  cursor: pointer;
  display: none;
}
.icon.icon-close {
  display: block;
}
:deep(.i-arrow path) {
  fill: var(--text-light) !important;
}
.i-arrow-invert {
  transform: rotate(180deg);
}
.btn-restore:hover {
  text-decoration: underline;
}
</style>