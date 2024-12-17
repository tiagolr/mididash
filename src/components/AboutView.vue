<script>
import Popup from './global/Popup.vue'
import ILogo from '../assets/logo.svg'
import semver from 'semver'
export default {
  components: {
    Popup,
    ILogo
  },

  data() {
    return {
      latestVersion: null,
      checking: true,
      error: false
    }
  },

  computed: {
    isLatestVersion: vm => vm.latestVersion && semver
      .gte(vm.$store.app.version, vm.latestVersion)
  },

  mounted () {
    fetch('https://api.github.com/repos/tiagolr/mididash/releases/latest')
      .then(res => res.json())
      .then(res => {
        this.latestVersion = res.tag_name
      })
      .catch(_ => {
        this.error = true
      })
      .finally(() => {
        this.checking = false
      })
  }
}
</script>

<template>
  <popup @close="$store.app.toggleAbout">
    <div class="about">
      <div class="text-center font-xl font-bold">
        <div>Mididash</div>
      </div>
      <i-logo style="width: 64px; height: 64px; margin: auto">
      </i-logo>
      <div class="text-center" style="font-size: 1rem; font-weight: normal;">
        v{{ $store.app.version }}
      </div>
      <div class="text-center ">
        <div v-if="checking">Checking for new versions...</div>
        <div v-else-if="error" class="error">Check for new version failed</div>
        <div v-else-if="isLatestVersion" class="success">Using latest version</div>
        <div v-else>
          New version available <strong class="success">{{ latestVersion }}</strong>
        </div>
      </div>
      <div class="text-center">
        <a href="https://github.com/tiagolr/mididash/releases/latest" target="_blank">
          Github
        </a>
      </div>
      <div class="text-center font-lighter">
        -- tilr © 2025 --
      </div>
    </div>
  </popup>
</template>


<style scoped>
.about {
  width: 200px;
  max-width: 80vw;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
.error {
  color: var(--error-content)
}
.success {
  color: var(--success-content)
}
</style>