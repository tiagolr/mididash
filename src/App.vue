<script>
import { listen } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'
import { saveWindowState, StateFlags, restoreStateCurrent } from '@tauri-apps/plugin-window-state'
import FlashMessages from './components/global/FlashMessages.vue'
import MainView from './components/MainView.vue'
import { EVT_WINDOW_SHOW, EVT_SETTINGS_CHANGE, EVT_PROJECT_NEW, EVT_ERROR, EVT_MIDI, EVT_FILE_OPEN, EVT_FILE_SAVE_AS, EVT_FILE_SAVE, EVT_SCRIPT_LOG, EVT_SCRIPT_ERROR, EVT_SHOW_ABOUT } from './globals'
import { invoke } from "@tauri-apps/api/core";
import AboutView from './components/AboutView.vue'

export default {
  components: {
    FlashMessages,
    MainView,
    AboutView,
  },
  data() {
    return {
      hidden: false, // for some reason stopping rendering when minimizing to tray greatly improves CPU resources
    }
  },
  async created() {
    listen(EVT_MIDI, this.$store.app.onGlobalEvent)
    listen(EVT_SCRIPT_ERROR, this.$store.app.onGlobalEvent)
    listen(EVT_SCRIPT_LOG, this.$store.app.onGlobalEvent)
    listen(EVT_ERROR, this.$store.app.handleError)
    listen(EVT_WINDOW_SHOW, this.onWindowShow)
    listen(EVT_SETTINGS_CHANGE, this.$store.app.getSettings)
    listen(EVT_PROJECT_NEW, this.$store.app.onProjectNew)
    listen(EVT_FILE_OPEN, this.$store.app.openFile)
    listen(EVT_FILE_SAVE_AS, this.$store.app.saveFileAs)
    listen(EVT_FILE_SAVE, this.$store.app.saveFile)
    listen(EVT_SHOW_ABOUT, this.$store.app.toggleAbout)

    getCurrentWindow().onCloseRequested(this.onCloseRequested)
    restoreStateCurrent(StateFlags.ALL)

    setInterval(this.$store.app.getMidiPorts, 2500)
    this.$store.app.getMidiPorts()
    this.$store.graph.init()
    this.$store.app.init()

    document.addEventListener('keydown', this.onGlobalKeydown)
    if (!import.meta.env.DEV) {
      document.addEventListener('contextmenu', (e) => { e.preventDefault() }) // stop right click menus
    }
  },
  async mounted () {
    await this.$store.app.onProjectNew() // fetch initial project and settings
    if (import.meta.env.DEV) {
      await invoke('new_devices_project') // fix hot reloading by resetting to a new project
    }
  },
  beforeUnmount () {
    document.removeEventListener('keydown', this.onGlobalKeydown)
  },
  methods: {
    async onWindowShow() {
      this.hidden = false
      await restoreStateCurrent(StateFlags.ALL)
      await getCurrentWindow().show()
      await getCurrentWindow().setFocus()
    },
    async onCloseRequested(event) {
      await saveWindowState(StateFlags.ALL)
      if (this.$store.app.settings.minimizeToTray) {
        event.preventDefault()
        getCurrentWindow().hide()
        this.hidden = true
      } else {
        event.preventDefault()
        this.$store.app
          .saveCurrentProject()
          .finally(() => {
            invoke('exit')
          })
      }
    },

    onGlobalKeydown (evt) {
      const el = document.activeElement
      if (el.tagName === 'INPUT' || el.tagName === 'TEXTAREA' || el.isContentEditable) {
		    return
	    }
      const key = evt.key.toUpperCase()

      if (key === 'DELETE') {
        this.$store.graph.removeSelected()
	    } else
        // accelerators not working properly on windows and macos
        // use global window events instead
        if (key === 'N' && evt.shiftKey && evt.ctrlKey) {
          this.$store.app.newBlankProject()
        } else if (key === 'N' && evt.ctrlKey) {
          this.$store.app.newDevicesProject()
        } else if (key === 'Q' && evt.ctrlKey) {
          this.$store.app.forceQuit()
        } else if (key === 'O' && evt.ctrlKey) {
          this.$store.app.openFile()
        } else if (key === 'S' && evt.ctrlKey && evt.shiftKey) {
          this.$store.app.saveFileAs()
        } else if (key === 'S' && evt.ctrlKey) {
          if (this.$store.app.settings.projectPath) {
            this.$store.app.saveFile()
          } else {
            this.$store.app.saveFileAs()
          }
        }
    },
  }
}
</script>

<template>
  <main
    v-if="!hidden"
    :class="{
      [$store.app.settings.theme || 'dark']: true,
      [$store.app.os]: true
    }"
  >
    <main-view>
    </main-view>
    <flash-messages>
    </flash-messages>
    <about-view v-if="$store.app.showAbout">
    </about-view>
  </main>
</template>

<style scoped>
main {
  width: 100vw;
  height: 100vh;
}
</style>
