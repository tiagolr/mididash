<script>
export default {
  props: {
    startX: Number,
    startY: Number,
    startW: Number,
    startH: Number,
  },
  emits: [
  'close',
  'stop-drag',
  'stop-resize'
  ],
  data() {
    return {
      position: { top: this.startY || 100, left: this.startX || 100 },
      size: { width: this.startW || 300, height: this.startH || 200 },
      isDragging: false,
      dragStart: { x: 0, y: 0 },
      resizeStart: { width: 0, height: 0, x: 0, y: 0 },
      currentHandle: null,
    };
  },
  computed: {
    resizeHandles() {
      return ['top', 'right', 'bottom', 'left', 'top-right', 'top-left', 'bottom-right', 'bottom-left'];
    },
  },
  methods: {
    getRect () {
      return {
        x: this.position.left, y: this.position.top,
        w: this.size.width, h: this.size.height
      }
    },
    clampWindow () {
      if (this.position.left < -this.size.width + 100) {
        this.position.left = -this.size.width + 100
      }
      if (this.position.left > window.innerWidth - 100) {
        this.position.left = window.innerWidth - 100
      }
      if (this.position.top < 0) {
        this.position.top = 0
      }
      if (this.position.top > window.innerHeight - 32) {
        this.position.top = window.innerHeight - 32
      }
    },
    startDrag(event) {
      this.isDragging = true;
      this.dragStart.x = event.clientX - this.position.left;
      this.dragStart.y = event.clientY - this.position.top;
      document.addEventListener('mousemove', this.onDrag, { passive: true });
      document.addEventListener('mouseup', this.stopDrag);
    },
    onDrag(event) {
      if (this.isDragging) {
        this.position.left = event.clientX - this.dragStart.x;
        this.position.top = event.clientY - this.dragStart.y;
        this.clampWindow()
      }
    },
    stopDrag() {
      this.isDragging = false;
      document.removeEventListener('mousemove', this.onDrag, { passive: true });
      document.removeEventListener('mouseup', this.stopDrag);
      this.$emit('stop-drag')
    },
    startResize(event, handle) {
      this.currentHandle = handle;
      this.resizeStart = {
        width: this.size.width,
        height: this.size.height,
        left: this.position.left,
        top: this.position.top,
        mouseX: event.clientX,
        mouseY: event.clientY,
      };

      event.stopPropagation()
      document.addEventListener('mousemove', this.onResize, { passive: true });
      document.addEventListener('mouseup', this.stopResize);
    },
    onResize(event) {
      const dx = event.clientX - this.resizeStart.mouseX;
      const dy = event.clientY - this.resizeStart.mouseY;

      switch (this.currentHandle) {
        case 'left':
          this.size.width = this.resizeStart.width - dx;
          this.position.left = this.resizeStart.left + dx;
          break;
        case 'right':
          this.size.width = this.resizeStart.width + dx;
          break;
        case 'top':
          this.size.height = this.resizeStart.height - dy;
          this.position.top = this.resizeStart.top + dy;
          break;
        case 'bottom':
          this.size.height = this.resizeStart.height + dy;
          break;
        case 'top-left':
          this.size.width = this.resizeStart.width - dx;
          this.position.left = this.resizeStart.left + dx;
          this.size.height = this.resizeStart.height - dy;
          this.position.top = this.resizeStart.top + dy;
          break;
        case 'top-right':
          this.size.width = this.resizeStart.width + dx;
          this.size.height = this.resizeStart.height - dy;
          this.position.top = this.resizeStart.top + dy;
          break;
        case 'bottom-left':
          this.size.width = this.resizeStart.width - dx;
          this.position.left = this.resizeStart.left + dx;
          this.size.height = this.resizeStart.height + dy;
          break;
        case 'bottom-right':
          this.size.width = this.resizeStart.width + dx;
          this.size.height = this.resizeStart.height + dy;
          break;
      }

      this.clampWindow()
    },
    stopResize() {
      document.removeEventListener('mousemove', this.onResize, { passive: true });
      document.removeEventListener('mouseup', this.stopResize);
      this.currentHandle = null;
      this.$emit('stop-resize')
    },
  },
};
</script>

<template>
  <div
    class="window"
    :style="{ top: position.top + 'px', left: position.left + 'px', width: size.width + 'px', height: size.height + 'px' }"
  >
    <div class="header" @mousedown="startDrag">
      <slot name="header"></slot>
      <svg width="19" height="19" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" class="btn-close" @click="$emit('close')">
        <path d="M16.875 7.125L7.125 16.875M7.125 7.125L16.875 16.875" stroke="var(--text-lighter)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" />
      </svg>
    </div>
    <div class="content">
      <slot></slot>
    </div>
    <div
      v-for="handle in resizeHandles"
      :key="handle"
      class="resize-handle"
      :class="`resize-handle-${handle}`"
      @mousedown="e => startResize(e, handle)"
    ></div>
  </div>
</template>

<style scoped>
.window {
  position: absolute;
  /* box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2); */
  /* background-color: rgba(0,0,0,0.85); */
  background: var(--foreground-alpha);
  border: 1px solid var(--border);
  z-index: 1;
  border-radius: var(--panel-radius);
  display: flex;
  flex-direction: column;
}

.header {
  min-height: 32px;
  /* color: var(--primary-content); */
  border-bottom: 1px solid var(--border);
  display: flex;
  align-items: center;
  padding: 0 var(--panel-padding);
  cursor: default;
  border-top-left-radius: var(--panel-radius);
  border-top-right-radius: var(--panel-radius);
}

.btn-close {
  margin-left: auto;
  cursor: pointer;
  margin-left: 4px;
}

.content {
  height: 100%;
  padding: var(--panel-padding);
  overflow-y: auto;
}

.resize-handle {
  position: absolute;
}

.resize-handle-left {
  left: -5px;
  top: 0;
  bottom: 0;
  width: 10px;
  cursor: ew-resize;
}

.resize-handle-right {
  right: -5px;
  top: 0;
  bottom: 0;
  width: 10px;
  cursor: ew-resize;
}

.resize-handle-top {
  top: -5px;
  left: 0;
  right: 0;
  height: 10px;
  cursor: ns-resize;
}

.resize-handle-bottom {
  bottom: -5px;
  left: 0;
  right: 0;
  height: 10px;
  cursor: ns-resize;
}

.resize-handle-top-left {
  top: -5px;
  left: -5px;
  width: 10px;
  height: 10px;
  cursor: nwse-resize;
}

.resize-handle-top-right {
  top: -5px;
  right: -5px;
  width: 10px;
  height: 10px;
  cursor: nesw-resize;
}

.resize-handle-bottom-left {
  bottom: -5px;
  left: -5px;
  width: 10px;
  height: 10px;
  cursor: nesw-resize;
}

.resize-handle-bottom-right {
  bottom: -5px;
  right: -5px;
  width: 10px;
  height: 10px;
  cursor: nwse-resize;
}
</style>