<template>
  <div
    class="node-group"
    :class="hover && 'hover'"
    :style="groupStyle"
    @mousedown="onMousedown"
  >
    <div style="position: relative; width: 100%; height: 100%;">
      <div class="outer" @mousedown="onMousedownOuter" @mouseenter="hover = true" @mouseleave="hover = false">
        <slot>
        </slot>
      </div>
    </div>
  </div>
</template>

<!--
  Custom version of vnodes group that allows to drag
  on the header while disabling the drag on the body
 -->
<script>
import drag from '../../lib/vnodes/src/mixins/drag'
export default {
  mixins: [
    drag
  ],
  props: {
    nodes: {
      type: Array,
      default: () => []
    },
    padding: { // additional area covered by group besides nodes minxy, maxxy
      type: Object,
      default: () => ({ left: 10, right: 10, top: 50, bottom: 10 })
    },
    disableDrag: Boolean,
  },
  data() {
    return {
      hover: false
    }
  },
  computed: {
    minX: vm => !vm.nodes.length ? 0 : vm.nodes.reduce((acc, node) => Math.min(acc, node.x), Infinity),
    maxX: vm => !vm.nodes.length ? 0 : vm.nodes.reduce((acc, node) => Math.max(acc, node.x + node.width), -Infinity),
    minY: vm => !vm.nodes.length ? 0 : vm.nodes.reduce((acc, node) => Math.min(acc, node.y), Infinity),
    maxY: vm => !vm.nodes.length ? 0 : vm.nodes.reduce((acc, node) => Math.max(acc, node.y + node.height), -Infinity),
    width: vm => vm.maxX - vm.minX,
    height: vm => vm.maxY - vm.minY,

    contentMargin: vm => vm.margin && ({
      margin: vm.margin + 'px',
      width: `calc(100% - ${vm.margin * 2}px)`,
      height: `calc(100% - ${vm.margin * 2}px)`
    }),

    groupStyle: vm => ({
      left: `${vm.minX - vm.padding.left}px`,
      top: `${vm.minY - vm.padding.top}px`,
      width: `${vm.width + vm.padding.left + vm.padding.right}px`,
      height: `${vm.height + vm.padding.top + vm.padding.bottom}px`
    })
  },
  methods: {
    onDrag ({ x,y }) {
      this.nodes.forEach(node => {
        node.x += x
        node.y += y
      })
    },
    onMousedown (e) {
      if (!this.disableDrag) {
        e.stopPropagation()
        e.preventDefault();
        this.startDrag(e);
      }
    },
    onMousedownOuter (e) {
      e.stopPropagation()
      e.preventDefault();
      this.startDrag(e);
    }
  }
}
</script>

<style scoped>
.node-group {
  position: absolute;
  display: inline-flex;
  border-radius: 7px;
  background: none;
  display: inline-block;
}
.outer {
  position: absolute;
  top: 0px;
  left: 50%;
  transform:
  translateX(-50%);
  padding: 8px;
}
</style>