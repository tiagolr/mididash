<script>
export default {
  props: {
    styles: Object,
  },
  emits: ['close'],
  mounted () {
    this.$refs.popup.focus()
    document.body.style.overflow = 'hidden'
  },
  unmounted () {
    document.body.style.overflow = ''
  }
}
</script>

<template>
  <Teleport to="body">
    <div
      ref="popup"
      class="popup"
      :class="$store.app.settings.theme || 'light'"
      tabindex="0"
      @keydown.esc.prevent.stop="$emit('close')"
    >
      <div class="background" @click="() => $emit('close')">
      </div>
      <div class="content" :style="styles?.content">
        <slot>
        </slot>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.popup {
  position: fixed;
  width: 100vw;
  height: 100vh;
  top: 0;
  left: 0;
  z-index: 100;
}

.background {
  background-color: #0008;
  position: absolute;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  animation: fadeIn 0.3s ease-out;
}

.content {
  padding: 1.5rem;
  background-color: var(--foreground-light);
  display: inline-block;
  border-radius: var(--panel-radius);
  position: absolute;
  top: 50%;
  left: 50%;
  max-width: 90vw;
  max-height: 90vh;
  transform: translate(-50%, -50%);
  box-shadow: 0px 0px 0px 0px rgba(0, 0, 0, 0.10), 0px 7px 16px 0px rgba(0, 0, 0, 0.10), 0px 29px 29px 0px rgba(0, 0, 0, 0.09), 0px 64px 39px 0px rgba(0, 0, 0, 0.05), 0px 114px 46px 0px rgba(0, 0, 0, 0.02), 0px 178px 50px 0px rgba(0, 0, 0, 0.00);
}

@keyframes fadeIn {
  0% { opacity: 0; }
  100% { opacity: 1; }
}

</style>