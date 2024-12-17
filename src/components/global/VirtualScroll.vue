<script>
export default {
  props: {
    items: {
      type: Array,
      required: true,
    },
    itemHeight: {
      type: Number,
      required: true,
    },
    containerHeight: {
      type: Number,
      default: 300,
    },
    buffer: {
      type: Number,
      default: 5,
    },
  },
  data() {
    return {
      scrollTop: 0,
    };
  },
  computed: {
    totalHeight() {
      return this.items.length * this.itemHeight;
    },
    startIndex() {
      return Math.max(0, Math.floor(this.scrollTop / this.itemHeight) - this.buffer);
    },
    endIndex() {
      return Math.min(
        this.items.length,
        Math.ceil((this.scrollTop + this.containerHeight) / this.itemHeight) + this.buffer
      );
    },
    visibleItems() {
      return this.items.slice(this.startIndex, this.endIndex);
    },
  },
  methods: {
    onScroll(event) {
      this.scrollTop = event.target.scrollTop;
    },
    getItemStyle(item) {
      const index = this.items.indexOf(item);
      return {
        position: "absolute",
        top: `${index * this.itemHeight}px`,
        height: `${this.itemHeight}px`,
        width: "100%",
      };
    },
    scrollToBottom() {
      const container = this.$refs.container;
      container.scrollTop = container.scrollHeight;
    },
  },
};
</script>

<template>
  <div
    ref="container"
    class="virtual-scroll"
    @scroll="onScroll"
  >
    <div :style="{ height: totalHeight + 'px'}">
      <div
        v-for="item in visibleItems"
        :key="item.id"
        class="virtual-scroll-item"
        :style="getItemStyle(item)"
      >
        <slot :item="item"></slot>
      </div>
    </div>
  </div>
</template>

<style scoped>
.virtual-scroll {
  height: 100%;
  overflow-y: auto;
  scroll-behavior: auto;
}
.virtual-scroll > div {
  position: relative;
  width: 100%;
}
</style>
