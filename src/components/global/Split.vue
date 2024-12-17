<script>
export default {
	props: {
		vertical: Boolean,
		horizontal: Boolean,
		resizeB: Boolean,
		usePercentage: Boolean,
		gap: {
			type: Number,
			default: 0,
		},
		init: {
			type: String,
			default: '50%',
		},
		min: {
			type: String,
			default: 'none',
		},
		max: {
			type: String,
			default: 'none',
		},
	},

	emits: [
		'drag-start',
		'drag-stop',
		'drag'
	],

	data() {
		return {
			isDragging: false,
			offset: this.init,
		};
	},

	computed: {
		offsetA() {
			if (typeof this.offset === 'string') {
				return this.offset;
			}
			return this.usePercentage
				? `${this.offset}%`
				: `${this.offset}px`
		},
		styleA() {
			return this.vertical ? {
				height: this.offsetA,
				minHeight: this.min,
				maxHeight: this.max,
				paddingTop: this.resizeB && this.gap/2+'px',
				paddingBottom: !this.resizeB && this.gap/2+'px'
			} : {
				width: this.offsetA,
				minWidth: this.min,
				maxWidth: this.max,
				paddingRight: !this.resizeB && this.gap/2+'px',
				paddingLeft: this.resizeB && this.gap/2+'px'
			}
		},
		styleB () {
			return {
				flex: 1,
				paddingTop: this.vertical && !this.resizeB && this.gap/2+'px',
				paddingBottom: this.vertical && this.resizeB && this.gap/2+'px',
				paddingRight: !this.vertical && this.resizeB && this.gap/2+'px',
				paddingLeft: !this.vertical && !this.resizeB && this.gap/2+'px',
			}
		}
	},

	methods: {
		dragStart() {
			this.$emit('drag-start')
			this.isDragging = true
			window.addEventListener('mousemove', this.dragging, { passive: true })
			window.addEventListener('touchmove', this.dragging, { passive: true })
			window.addEventListener('touchend', this.dragStop, { passive: true, once: true })
			window.addEventListener('mouseup', this.dragStop, { passive: true, once: true });
		},

		dragStop() {
			window.removeEventListener('mousemove', this.dragging, { passive: true })
			window.removeEventListener('touchmove', this.dragging, { passive: true })
			this.isDragging = false
			this.$emit('drag-stop')
		},

		mouseOffset({ pageX, pageY }) {
			const { container } = this.$refs;
			const containerOffset = this.getPosition(container);
			let offset;

			if (this.vertical) {
				offset = pageY - containerOffset.y;
				offset = Math.min(offset, container.offsetHeight);
			} else {
				offset = pageX - containerOffset.x;
				offset = Math.min(offset, container.offsetWidth);
			}

      const containerSize = this.vertical
        ? container.offsetHeight
        : container.offsetWidth

			const val = this.resizeB
        ? Math.max(containerSize - offset, 0)
        : Math.max(offset, 0)

			return this.usePercentage ? val / containerSize * 100 : val
		},

		dragging(event) {
			this.offset = this.mouseOffset(event.touches?.[0] || event)
			this.$emit('drag')
		},

		getPosition(element) {
			let xPosition = 0;
			let yPosition = 0;

			while (element) {
				xPosition += element.offsetLeft - element.scrollLeft + element.clientLeft;
				yPosition += element.offsetTop - element.scrollTop + element.clientTop;
				element = element.offsetParent;
			}

			return { x: xPosition, y: yPosition };
		}
	},
};
</script>

<template>
  <div ref="container" class="split-view" :class="{vertical}">
    <div class="side-a" :class="{'is-locked': isDragging}" :style="!resizeB ? styleA : styleB">
      <slot name="A">
      </slot>
    </div>
    <span
      class="handle"
      :class="{vertical, horizontal: !vertical}"
      @mousedown.prevent="dragStart"
      @touchstart.prevent="dragStart"
    ></span>
    <div class="side-b" :class="{ 'is-locked': isDragging }" :style="!resizeB ? styleB : styleA">
      <slot name="B">
      </slot>
    </div>
  </div>
</template>

<style scoped>
.split-view {
	position: relative;
	display: flex;
	width: 100%;
	height: 100%;
}
.split-view.vertical {
	flex-direction: column;
}
.handle {
	user-select: none;
	box-sizing: border-box;
	transition: all 0.3s ease;
	z-index: 1;
	/* background: padding-box #000a; */
	background: padding-box none;
}
.handle.horizontal {
  width: 10px;
  border-left: 5px solid transparent;
  border-right: 5px solid transparent;
  margin: 0 -5px;
  cursor: col-resize;
}
.handle:hover, .handle:active {
	/* border-color: #0004; */
	border-color: none;
}
.handle:active {
	border-width: 4px;
}
.handle.vertical {
  height: 10px;
  border-top: 5px solid transparent;
  border-bottom: 5px solid transparent;
  margin: -5px 0;
  cursor: row-resize;
}

.side-a {
	overflow: auto;
	display: flex;
	flex-direction: column;
}
.side-b {
  overflow: auto;
	display: flex;
	flex-direction: column;
}

.side-a.is-locked {
  pointer-events: none;
}
.side-b.is-locked {
  pointer-events: none;
}
</style>