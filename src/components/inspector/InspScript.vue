<script>
import { VAceEditor } from 'vue3-ace-editor';
import 'ace-builds/src-noconflict/mode-lua';
import 'ace-builds/src-noconflict/theme-cobalt';
import 'ace-builds/src-noconflict/theme-github_light_default';
import { EVT_SCRIPT_LOG, EVT_SCRIPT_ERROR, EXPAND_CODE_WINDOW } from '../../globals'
import IBroom from '../../assets/broom.svg'
import IClose from '../../assets/close.svg'
import IConfig from '../../assets/wrench.svg'
import IExpand from '../../assets/expand.svg'
import ContextMenu from '../global/ContextMenu.vue';

const MAX_LOGS = 250;
export default {
  components: {
    ContextMenu,
    VAceEditor,
    IBroom,
    IClose,
    IConfig,
    IExpand,
  },
  props: {
    device: Object
  },
  data() {
    return {
      script: this.device.script,
      testBytes: this.device.testBytes || '0x80 60 127',
      logs: [],
      scriptMenu: {
        visible: false,
        x: 0,
        y: 0,
      }
    }
  },
  computed: {
    showGutter: vm => vm.$store.app.settings.scriptShowLineNumbers,
    scriptMenuItems: vm => [
      { id: 'show-gutter', label: 'Show line numbers', type: 'checkbox', checked: vm.showGutter },
      { id: 'save-template', label: 'Save as template' },
    ],
    editorTheme: vm => vm.$store.app.isLightTheme
      ? 'github_light_default'
      : 'cobalt'
  },
  watch: {
    device () {
      this.script = this.device.script
      this.logs = []
    },
    'device.script' (script) {
      this.script = script
    },
    'device.testBytes' (s) {
      this.testBytes = s
    }
  },
  created () {
    this.$store.app.emitter.on(EVT_SCRIPT_LOG, this.logEvent)
    this.$store.app.emitter.on(EVT_SCRIPT_ERROR, this.logEventError)
    if (this.device.error) {
      this.logEventError({ id: this.device.id, error: this.device.error })
    }
  },
  beforeUnmount () {
    this.$store.app.emitter.off(EVT_SCRIPT_LOG, this.logEvent)
    this.$store.app.emitter.off(EVT_SCRIPT_ERROR, this.logEventError)
  },
  methods: {
    async saveScript () {
      await this.$store.graph.setDeviceData(this.device.id, 'script', this.script)
      await this.$store.graph.setDeviceProperty(this.device.id, 'testBytes', this.testBytes)
      await this.$store.graph.setDeviceProperty(this.device.id, 'error', null)
      this.logSuccess('Script saved')
    },
    async testScript () {
      await this.saveScript()
      const bytes = this.testBytes.split(/\s+/).map(s => parseInt(s)).filter(i => !isNaN(i))
      await this.$store.graph.setDeviceData(this.device.id, 'testBytes', bytes)
      this.logSuccess(`Testing ${bytes}`)
      const res = await this.$store.graph.getDeviceData(this.device.id, "testResult")
      res.forEach(entry => {
        this.logSuccess(entry)
        if (entry[0] && !this.device.outPorts.find(p => p.id === entry[0])) {
          this.logWarn(`Port ${entry[0]} not found`)
        }
      })
      if (!res.length) {
        this.logWarn('No output found')
      }
    },
    log (message, type = 'info') {
      this.logs.push({
        id: Math.random().toString(),
        type,
        message
      })
      this.logs = this.logs.slice(-MAX_LOGS)
      setTimeout(() => {
        this.$refs.logs.scrollTop = this.$refs.logs.scrollHeight;
      }, 0);
    },
    logError (message) {
      this.log(message, 'error')
    },
    logSuccess (message) {
      this.log(message, 'success')
    },
    logWarn (message) {
      this.log(message, 'warn')
    },
    logEvent (payload) {
      if (payload.id === this.device.id) {
        this.log(payload.message)
      }
    },
    logEventError(payload) {
      if (payload.id === this.device.id) {
        this.logError(payload.error
          .replace(/stack traceback:(.|(\r\n|\n|\r))*/, ' ')
          .replace(/: \[.*\]/, ' '))
      }
    },
    onEnterKeyEditor(evt) {
      if (evt.ctrlKey || evt.metaKey) {
        this.testScript()
      }
    },
    clearLogs () {
      this.logs = []
    },
    addOutPort () {
      this.$store.graph.addOutport(this.device.id)
      setTimeout(() => {
        this.$refs.outports.scrollTop = this.$refs.outports.scrollHeight;
      }, 0);
    },
    updatePortId (index) {
      const currentId = this.device.outPorts[index].id
      const newid = this.$refs.outPort[index].innerText.trim()
      if (newid === currentId) return
      if (newid === '') {
        this.$refs.outPort[index].innerText = currentId
        return
      }
      this.$store.graph.setOutportId(this.device.id, currentId, newid)
    },
    removePort(id) {
      this.$store.graph.removeOutport(this.device.id, id)
    },
    onPortEnterPress (event) {
      event.target.blur()
    },
    toggleScriptMenu () {
      const rect = this.$refs.configButton.$el.getBoundingClientRect()
      this.scriptMenu.x = rect.x
      this.scriptMenu.y = rect.y + rect.height
      this.scriptMenu.visible = !this.scriptMenu.visible
    },
    onSelectScriptMenu (id) {
      if (id === 'save-template') {
        this.$store.app.saveScriptTemplate({
          name: this.device.name,
          outPorts: this.device.outPorts.map(p => p.id),
          script: this.script,
        })
        this.$store.app.showSuccess('Template saved')
      } else if (id === 'show-gutter') {
        this.$store.app.setSettings({ scriptShowLineNumbers: !this.showGutter })
      }
      this.scriptMenu.visible = false
    },
    resetScriptState () {
      this.$store.graph.setDeviceData(this.device.id, "reset-state", {})
    },
    expandCodeWindow () {
      this.$store.app.emitter.emit(EXPAND_CODE_WINDOW, this.device.id)
    }
  }
}
</script>

<template>
  <div class="mt-1rem mb-025rem flex-center">
    <div class="font-lighter ">Out ports</div>
    <div class="flex-right cursor-pointer" @click="addOutPort">+ Add</div>
  </div>
  <div v-if="device.outPorts.length" class="list compact panel mt-025rem">
    <div ref="outports" class="overflow" style="max-height: 65px; scroll-behavior: auto;">
      <div v-for="port,i in device.outPorts" :key="i" class="list-item">
        <div
          ref="outPort"
          contenteditable
          style="outline: none; cursor:text"
          :data-outport="i"
          class="flex-1"
          @keydown.enter.prevent="$refs.outPort[i].blur()"
          @blur="updatePortId(i)"
        >
          {{ port.id }}
        </div>
        <div class="flex-right cursor-pointer" style="padding-left: 10px" @click="removePort(port.id)">
          <i-close class="icon-remove">
          </i-close>
        </div>
      </div>
    </div>
  </div>
  <div class="font-lighter mt-1rem mb-025rem flex-center gap-4">
    <div>Script</div>
    <div class="flex gap-4 flex-right">
      <div title="Expand" @click="expandCodeWindow">
        <i-expand class="icon icon-clear">
        </i-expand>
      </div>
      <i-config ref="configButton" class="icon icon-clear" @click="toggleScriptMenu">
      </i-config>
    </div>
    <context-menu
      v-if="scriptMenu.visible"
      :start-x="scriptMenu.x"
      :start-y="scriptMenu.y"
      :items="scriptMenuItems"
      @close="scriptMenu.visible = false"
      @select="onSelectScriptMenu"
    ></context-menu>
  </div>
  <div class="editor-container" @keydown.enter="onEnterKeyEditor">
    <v-ace-editor
      v-model:value="script"
      lang="lua"
      :theme="editorTheme"
      style="height: 100%;background: none;"
      :options="{
        showGutter: showGutter,
        displayIndentGuides: false,
        highlightActiveLine: false,
        highlightGutterLine: false,
        useSoftTabs: false,
        showPrintMargin: false,
        tabSize: 2,
      }"
    />
  </div>
  <div class="mt-05rem flex gap-8">
    <button class="button" @click="saveScript">
      Save
    </button>
    <button class="button" :title="'Ctrl+Enter'" @click="testScript">
      Test
    </button>
    <button class="button" @click="resetScriptState">
      Reset
    </button>
  </div>
  <div class="mt-05rem" title="Test bytes">
    <input v-model="testBytes" type="text" class="input" placeholder="Test bytes" @keydown.enter="onEnterKeyEditor">
  </div>
  <div class="font-lighter mt-1rem mb-025rem flex-center">
    <div>Logs</div>
    <div class="flex-right">
      <i-broom class="icon icon-clear" @click="clearLogs">
      </i-broom>
    </div>
  </div>
  <div class="logs">
    <div ref="logs" class="overflow">
      <div v-for="l in logs" :key="l.id" class="log" :class="l.type">
        {{ l.message }}
      </div>
    </div>
  </div>
</template>


<style scoped>
:deep(.ace_editor) {
  font-family: "Consolas", "Liberation Mono", "Monaco", monospace;
}
input {
  cursor: text;
}
:deep(.ace_gutter) {
  background: var(--input)
}
.log.success {
  color: var(--monitor-green);
}
.log.error {
  color: var(--monitor-red);
}
.log.warn {
  color: var(--monitor-yellow);
}
.editor-container {
  padding: 8px;
  background: var(--input);
  border-radius: var(--panel-radius);
  height: 200px;
  resize: vertical;
  overflow: auto;
}
.logs {
  font-family: "Consolas", "Liberation Mono", "Monaco", monospace;
  font-size: 12px;
  padding: 8px;
  background: var(--input);
  border-radius: var(--panel-radius);
  user-select: auto;
  -webkit-user-select: auto;
  resize: vertical;
  display: flex;
  flex-direction: column;
  height: 100px;
  overflow: auto;
}
.log {
  cursor: text;
}
.icon-clear {
  width: 17px;
  height: 17px;
  cursor: pointer;
}
:deep(.icon path) {
  fill: var(--text);
}
.icon-remove {
  width: 17px;
  height: 17px;
  margin-bottom: -5px;
  cursor: pointer;
  display: none;
}
.list-item:hover .icon-remove {
  display: block;
}
</style>