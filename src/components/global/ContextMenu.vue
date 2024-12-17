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
    }
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

    if (top + height > window.innerHeight - PADDING) {
      top = window.innerHeight - height - PADDING;
    }
    if (left + width > window.innerWidth - PADDING) {
      left = window.innerWidth - width - PADDING;
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
  <div class="menu" :style="{left: x+'px', top: y+'px'}" @click.stop.prevent>
    <div v-for="item in items" :key="item.id" class="item" @click="$emit('select', item.id)">
      <checkbox v-if="item.type === 'checkbox'" :checked="item.checked">
      </checkbox>
      <div v-else>
      </div>
      <div>{{ item.label }}</div>
    </div>
  </div>
</template>


<style scoped>
.menu {
  position: fixed;
  z-index: 100;
  min-width: 100px;
  padding: 4px;
  background: var(--foreground-lighter);
  border-radius: var(--panel-radius);
  box-shadow: 2px 2px 7px 2px #0008;
}
.item {
  color: var(--copy);
  white-space: nowrap;
  display: flex;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
  height: 30px;
  padding-right: 20px;
  padding-left: 4px;
  border-radius: var(--panel-radius);
  cursor: pointer;
}
.item > div:first-child {
  width: 16px;
  min-height: 1px;
}
.item:hover {
  background: #fff3;
}
</style>