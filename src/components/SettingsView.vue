<script>
import Popup from './global/Popup.vue'
import Checkbox from './global/forms/Checkbox.vue';
export default {
  components: {
    Popup,
    Checkbox
  },

  data() {
    return {
    }
  },
}
</script>

<template>
  <popup @close="$store.app.toggleSettings">
    <div class="settings">
      <div class="font-xl font-bold">
        <div>Settings</div>
      </div>
      <div>
        <div class="flex-column gap-05rem">
          <div class="flex-center gap-05rem" @click="$store.app.setSettings({ startMinimized: !$store.app.settings.startMinimized })">
            <checkbox :checked="$store.app.settings.startMinimized">
            </checkbox>
            <div>Minimize to tray on startup</div>
          </div>
        </div>
      </div>
    </div>
  </popup>
</template>


<style scoped>
.settings {
  width: 200px;
  max-width: 80vw;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
</style>