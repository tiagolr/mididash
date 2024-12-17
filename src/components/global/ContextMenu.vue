<script>
import Checkbox from './forms/Checkbox.vue';
const PADDING = 8
export default {
  components: {
    Checkbox
  },
  props: {
    startX: {
      type: Number,
      default: -10000,
    },
    startY: {
      type: Number,
      default: -10000
    },
    items: {
      type: Array,
      default: () => []
    },
    ignorePadding: Boolean,
    noShadow: Boolean,
  },
  emits: [
    'select',
    'close'
  ],
  data() {
    return {
      x: -10000,
      y: -10000,
    }
  },
  mounted () {
    setTimeout(() => {
      document.addEventListener('click', this.onClickOutside)
      window.addEventListener('resize', this.onClickOutside)
    }, 0);
    const { width, height } = this.$el.getBoundingClientRect()

    let left = this.startX
    let top = this.startY

    const padding = this.ignorePadding ? 0 : PADDING

    if (top + height > window.innerHeight - padding) {
      top = window.innerHeight - height - padding;
    }
    if (left + width > window.innerWidth - padding) {
      left = window.innerWidth - width - padding;
    }

    this.x = left
    this.y = top
  },
  beforeUnmount () {
    document.removeEventListener('click', this.onClickOutside)
    window.removeEventListener('resize', this.onClickOutside)
  },
  methods: {
    onClickOutside () {
      this.$emit('close', null)
    }
  }
}
</script>

<template>
  <div class="menu" :class="noShadow && 'no-shadow'" :style="{left: x+'px', top: y+'px'}" @click.stop.prevent>
    <div v-for="item in items" :key="item.id" class="item" :class="item.type" @click="$emit('select', item.id)">
      <checkbox v-if="item.type === 'checkbox'" :checked="item.checked">
      </checkbox>
      <div v-else>
      </div>
      <div v-if="item.type !== 'separator'" class="flex">
        <div>{{ item.label }}</div>
      </div>
      <div v-if="item.tip" class="flex-right font-lighter" style="padding-left: 1rem">
        {{ item.tip }}
      </div>
    </div>
  </div>
</template>


<style scoped>
.menu {
  padding: 4px;
  position: fixed;
  z-index: 100;
  min-width: 100px;
  background: var(--foreground-lighter);
  border-radius: var(--panel-radius);
  box-shadow: 2px 2px 7px 2px #0008;
}
.menu.no-shadow {
  box-shadow: none;
}
.item {
  color: var(--copy);
  white-space: nowrap;
  display: flex;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
  height: 30px;
  padding-right: 4px;
  padding-left: 4px;
  border-radius: var(--panel-radius);
  cursor: pointer;
}
.item.separator {
  height: 1px;
  margin-left: -4px;
  margin-right: -4px;
  margin-top: 4px;
  margin-bottom: 4px;
  border-bottom: 1px solid #fff3;
  pointer-events: none;

}
.item > div:first-child {
  width: 16px;
  min-height: 1px;
}
.item:hover {
  background: #fff3;
}
</style>