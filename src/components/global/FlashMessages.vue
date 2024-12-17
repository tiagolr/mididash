<script>
import appStore from '../../stores/app'
export default {
  data() {
    return {
      store: {
        app: appStore()
      },
      flashMessages: []
    }
  },
}
</script>


<template>
  <div class="messages">
    <TransitionGroup name="list" tag="span">
      <div
        v-for="msg,i in store.app.flashMessages" :key="`message${i}`"
        class="message"
        :class="msg.type"
      >
        <div>{{ msg.text }}</div>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.list-enter-active,
.list-leave-active {
  transition: all 0.5s ease;
}
.list-enter-from,
.list-leave-to {
  opacity: 0;
  transform: translateY(30px);
}

.messages {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  z-index: 2000;
  pointer-events: none;
}

.message {
  pointer-events: none;
  width: fit-content;
  min-height: 32px;
  max-width: 800px;
  margin: 12px auto;
  padding: 0 12px;
  display: flex;
  align-items: center;
  gap: 12px;
  background-color: white;
  border-radius: var(--node-radius);
}

.message.info {
  background: #08a1dd;
  color: #dbe8ee
}

.message.error {
  background-color: var(--error);
  color: 1px solid var(--error-content);
}

.message.success {
  background-color: var(--success);
  color: 1px solid var(--success-content);
}
</style>