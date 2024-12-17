<script>
import Popup from './global/Popup.vue';
import ILogo from '../assets/logo.svg'
export default {
  components: {
    Popup,
    ILogo
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
        <div>Check for new versions on</div>
        <div><a href="https://www.google.com" target="_blank">Github</a></div>
      </div>
      <div class="text-center font-light">
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
  gap: 16px
}
a {
  color: var(--primary-content);
}
</style>