<script>
import { VAceEditor } from 'vue3-ace-editor';
import 'ace-builds/src-noconflict/mode-lua';
import 'ace-builds/src-noconflict/theme-cobalt';
import 'ace-builds/src-noconflict/theme-github_light_default';
import { EVT_SCRIPT_LOG, EVT_SCRIPT_ERROR } from '../globals'
import IBroom from '../assets/broom.svg'

const MAX_LOGS = 250;
export default {
  components: {
    VAceEditor,
    IBroom,
  },
  props: {
    device: Object
  },
  data() {
    return {
      script: this.device?.script || '',
      testBytes: this.device?.testBytes || '0x80 60 127',
      logs: [],
    }
  },
  computed: {
    editorTheme: vm => vm.$store.app.isLightTheme
      ? 'github_light_default'
      : 'cobalt'
  },
  watch: {
    device () {
      this.script = this.device?.script || ''
      this.logs = []
      if (this.device?.error) {
        this.logEventError({ id: this.device.id, error: this.device.error })
      }
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
    if (this.device?.error) {
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
    resetScriptState () {
      this.$store.graph.setDeviceData(this.device.id, "reset-state", {})
    }
  }
}
</script>

<template>
  <div class="code-view">
    <div class="editor-container" @keydown.enter="onEnterKeyEditor">
      <v-ace-editor
        v-model:value="script"
        lang="lua"
        :theme="editorTheme"
        style="height: 100%;background: none;"
        :options="{
          showGutter: true,
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
      <input v-model="testBytes" type="text" class="input test-bytes" placeholder="Test bytes" @keydown.enter="onEnterKeyEditor">
    </div>
    <div class="font-lighter mt-05rem mb-025rem flex-center">
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
  </div>
</template>


<style scoped>
.code-view {
  display: flex;
  flex-direction: column;
  height: 100%;
}
:deep(.ace_editor) {
  font-family: "Consolas", "Liberation Mono", "Monaco", monospace;
}
input {
  cursor: text;
}
.test-bytes {
  background: none;
  border: 1px solid var(--border);
}
:deep(.ace_gutter) {
  background: none;
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
  background: none;
  border-radius: var(--panel-radius);
  height: 200px;
  overflow: auto;
  /* border: 1px solid var(--border); */
  flex-grow: 1;
}
.logs {
  font-family: "Consolas", "Liberation Mono", "Monaco", monospace;
  font-size: 12px;
  padding: 8px;
  background: none;
  border-radius: var(--panel-radius);
  user-select: auto;
  -webkit-user-select: auto;
  display: flex;
  flex-direction: column;
  height: 100px;
  overflow: auto;
  border: 1px solid var(--border);
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