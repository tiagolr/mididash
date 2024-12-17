<script>
export default {
  props: {
    device: Object
  },
  data() {
    return {
      trigger: this.device.trigger
    }
  },
  methods: {
    saveTrigger () {
      this.$store.graph.setDeviceProperty(this.device.id, 'trigger', this.trigger)
    },
    saveBytes () {
      this.trigger.bytes = this.$refs.bytes.innerText
      this.saveTrigger()
    },
    onEnterBytes (evt) {
      if (evt.ctrlKey || evt.metaKey) {
        this.saveBytes()
        this.sendBytes()
      }
    },
    sendBytes () {
      const bytes = this.trigger.bytes
        .split(/\s+/)
        .map(s => parseInt(s))
        .filter(i => !isNaN(i))
        .map(i => i % 256)

      this.$store.app.hubProcess({
        ts: Date.now(),
        bytes,
        from: this.device.id,
        to: '*',
        fromPort: '*',
        toPort: '*'
      })
    }
  }
}
</script>

<template>
  <div class="mt-1rem font-lighter mb-025rem">
    Octaves
  </div>
  <div class="flex-center gap-8">
    <div>Start</div>
    <select v-model="trigger.noteStart" class="select">
      <option :value="0">C-1</option>
      <option :value="1">C0</option>
      <option :value="2">C1</option>
      <option :value="3">C2</option>
      <option :value="4">C3</option>
      <option :value="5">C4</option>
      <option :value="6">C5</option>
      <option :value="7">C6</option>
      <option :value="8">C7</option>
      <option :value="9">C8</option>
      <option :value="10">C9</option>
    </select>
    <div style="margin-left: 4px">Range</div>
    <select v-model="trigger.noteRange" class="select">
      <option :value="1">1</option>
      <option :value="2">2</option>
      <option :value="3">3</option>
      <option :value="4">4</option>
      <option :value="5">5</option>
      <option :value="6">6</option>
      <option :value="7">7</option>
      <option :value="8">8</option>
    </select>
  </div>
  <div class="mt-1rem font-lighter mb-025rem">
    Bytes
  </div>
  <div class="panel">
    <div
      ref="bytes"
      class="overflow bytes"
      contenteditable
      @blur="saveBytes"
      @keydown.enter="onEnterBytes"
    >
      {{ device.trigger.bytes }}
    </div>
  </div>
  <button class="button mt-05rem" @click="sendBytes">
    Send
  </button>
</template>


<style scoped>
.panel {
  padding: 0.5rem !important;
  max-height: 180px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}
.bytes {
  outline: none;
  cursor: text;
}
</style>