<script>
export default {
  props: {
    modelValue: Number,
    min: {
      type: Number,
      default: -Infinity
    },
    max: {
      type: Number,
      default: Infinity
    },
    placeholder: String,
    showPlaceholderOnMin: Boolean,
  },
  emits: ['update:modelValue', 'change'],
  data() {
    return {
      timeout: null,
      incrementSpeed: 0,
    }
  },
  computed: {
    value: {
      get() {
        return this.modelValue
      },
      set(value) {
        this.$emit('update:modelValue', Math.max(this.min, (Math.min(this.max, value))))
      }
    }
  },
  watch: {
    modelValue: {
      immediate: true,
      handler (v) {
        if (this.showPlaceholderOnMin) {
          this.$nextTick(() => {
            this.$refs.input.value = v === this.min ? '' : v
          })
        }
      }
    }
  },
  methods: {
    increment () {
      if (this.value <= this.max - 1) {
        this.value = this.value + 1
      }
      clearTimeout(this.timeout)
      this.incrementSpeed += 1
      this.timeout = setTimeout(this.increment, Math.max(5, 250 / this.incrementSpeed));
    },
    decrement () {
      if (this.value >= this.min + 1) {
        this.value = this.value - 1
      }
      clearTimeout(this.timeout)
      this.incrementSpeed += 1
      this.timeout = setTimeout(this.decrement, Math.max(5, 250 / this.incrementSpeed));
    },
    clearTimeout () {
      if (this.incrementSpeed !== 0) {
        this.$emit('change')
      }
      this.incrementSpeed = 0
      clearTimeout(this.timeout)
    },
    onInput() {
      const value = this.$refs.input.value
      if (value === '' || (this.showPlaceholderOnMin && parseInt(value) === this.min)) {
        this.$refs.input.value = ''
      } else {
        this.$refs.input.value = this.value
      }
    },
  }
}
</script>

<template>
  <div class="flex number-input">
    <input
      ref="input"
      v-model="value"
      type="number"
      :min="min"
      :max="max"
      :placeholder="placeholder"
      @input="onInput"
      @blur="$emit('change')"
      @keydown.enter="$refs.input.blur"
    >
    <div>
      <div @mousedown="increment" @mouseup="clearTimeout" @mouseleave="clearTimeout">
        <svg width="11" height="11" viewBox="0 0 5.292 5.292" xmlns="http://www.w3.org/2000/svg" style="transform: rotate(180deg)">
          <path style="stroke:none;" d="m1.302 3.046-.525-.91-.526-.911H2.354l-.526.91z" transform="translate(.799 -.383) scale(1.41818)" />
        </svg>
      </div>
      <div @mousedown="decrement" @mouseup="clearTimeout" @mouseleave="clearTimeout">
        <svg width="11" height="11" viewBox="0 0 5.292 5.292" xmlns="http://www.w3.org/2000/svg">
          <path style="stroke:none;" d="m1.302 3.046-.525-.91-.526-.911H2.354l-.526.91z" transform="translate(.799 -.383) scale(1.41818)" />
        </svg>
      </div>
    </div>
  </div>
</template>


<style scoped>
svg {
  display:block;
  fill:var(--text-light);
}
svg:hover path {
  fill: var(--text-lighter);
}
input {
  all: unset;
  background: none !important;
  width: 100%;
  outline: none;
  border: none;
  color: var(--text);
  font-size: normal;
  text-align: right;
  appearance: none;
  margin-right: 4px;
  -webkit-appearance: none;
  cursor: text;
}
input::placeholder {
  color: var(--text-lighter)
}
input[type="number"]::-webkit-outer-spin-button,
input[type="number"]::-webkit-inner-spin-button {
  -webkit-appearance: none;
  appearance: none;
}
</style>