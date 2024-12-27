<template>
  <div
    class="node"
    :style="nodeStyle"
    @mousedown.left="onMousedown"
    @touchstart="onMousedown"
  >
    <div v-if="!$slots.default" class="default-label">
      {{ data.id }}
    </div>
    <slot>
    </slot>
  </div>
</template>

<script>
import dragMixin from '../mixins/drag.js'
export default {
  mixins: [
    dragMixin
  ],
  props: {
    data: Object, // { x, y, width, height }
    margin: { // margin allows to display borders, shadow etc without clipping contents
      type: Number,
      default: 10,
    },
    fit: {
      type: Boolean,
      default: true,
    },
    useDrag: { // use default drag behavior
      type: Boolean,
      default: true
    },
    snapTo: {
      type: Number,
      default: 0 // default no snap
    }
  },
  emits: [
    'drag'
  ],
  data() {
    return {
      dragStartX: 0,
      dragStartY: 0,
      dragAccX: 0,
      dragAccY: 0
    }
  },
  computed: {
    nodeStyle: vm => ({
      left: vm.data.x + 'px',
      top: vm.data.y + 'px',
    })
  },
  mounted () {
    if (this.fit) {
      this.fitContent()
    }
  },
  methods: {
    onDrag ({ x,y }) {
      this.dragAccX += x
      this.dragAccY += y
      if (this.snapTo) {
        // eslint-disable-next-line vue/no-mutating-props
        this.data.x = Math.round((this.dragStartX + this.dragAccX) / this.snapTo) * this.snapTo - 4
        // eslint-disable-next-line vue/no-mutating-props
        this.data.y = Math.round((this.dragStartY + this.dragAccY) / this.snapTo) * this.snapTo - 4
      } else {
        // eslint-disable-next-line vue/no-mutating-props
        this.data.x = this.dragStartX + this.dragAccX
        // eslint-disable-next-line vue/no-mutating-props
        this.data.y = this.dragStartY + this.dragAccY
      }
      this.$emit('drag', { x, y })
    },
    /**
     * Update node width/height from the contents size
     */
    fitContent () {
      // eslint-disable-next-line vue/no-mutating-props
      this.data.width = this.$el.offsetWidth
      // eslint-disable-next-line vue/no-mutating-props
      this.data.height = this.$el.offsetHeight
    },
    onMousedown (e) {
      if (this.useDrag) {
        e.stopPropagation() // prevent viewport drag
        this.dragStartX = this.data.x
        this.dragStartY = this.data.y
        this.dragAccX = 0
        this.dragAccY = 0
        this.startDrag(e)
      }
    }
  }
}
</script>

<style>
.node {
  display: inline-flex;
  flex-direction: column;
  position: absolute;
  background-color: rgba(100, 200, 100, .9);
  border-radius: 7px;
}

.node .default-label {
  font-weight: bold;
  width: auto;
  height: auto;
  min-width: 30px;
  min-height: 30px;
  line-height: 30px;
  padding: 10px;
  text-align: center;
}
</style>