<template>
  <span class="port">
    <slot></slot>
  </span>
</template>

<script>
/**
 * Offsets edges position when placed inside a node
 */
export default {
  props: {
    id: String, // optional id
    startOffset: Object, // {x, y}, contents center if null
    edgesFrom: {
      type: Array,
      default: () => [],
    },
    edgesTo: {
      type: Array,
      default: () => [],
    }
  },
  data() {
    return {
      offset: { x: 0, y: 0 },
    }
  },
  computed: {
    // use a computed property by merging all edge ids
    // to update position, avoids recalculations
    edgeIds: vm => vm.edgesFrom
      .map(e => e.id).concat(vm.edgesTo
      .map(e => e.id)).join(' ')
  },
  watch: {
    edgeIds: 'updatePosition'
  },
  mounted () {
    this.updatePosition()
  },
  methods: {
    /**
     * Calculate html offset of this element
     * and update edges anchors to this element offset
     */
    updatePosition () {
      this.offset = this.startOffset || {
        x: this.$el.offsetWidth / 2,
        y: this.$el.offsetHeight / 2,
      }
      const rectPort = this.$el.getBoundingClientRect()
      const rectNode = this.$el.closest('.node')?.getBoundingClientRect()
      if (!rectNode) return
      const diffx = rectPort.left - rectNode.left
      const diffy = rectPort.top - rectNode.top

      const panzoom = this.$el.closest(".screen")?.querySelector('.svg-pan-zoom_viewport')
      if (!panzoom) return
      const zoom = new DOMMatrix(window.getComputedStyle(panzoom).transform).a
      this.offset.x += diffx / zoom
      this.offset.y += diffy / zoom

      this.edgesFrom.forEach(edge => {
        Object.assign(edge.fromAnchor, this.offset)
      })
      this.edgesTo.forEach(edge => {
        Object.assign(edge.toAnchor, this.offset)
      })
    }
  },
}
</script>

<style>
</style>