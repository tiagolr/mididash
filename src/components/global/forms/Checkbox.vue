<script>
export default {
  props: {
    checked: Boolean,
    disabled: Boolean,
  },
  emits: [
    'click',
    'focus',
    'blur'
  ]
}
</script>

<template>
  <div
    class="checkbox"
    :class="{checked, disabled}"
    tabindex="0"
    role="button"
    @click="() => !disabled && $emit('click', !checked)" @keydown.space="() => !disabled && $emit('click', !checked)"
  >
    <div class="inner" @focus="$emit('focus')" @blur="$emit('blur')">
      <svg
        v-if="checked"
        width="14" height="14" fill="none"
        xmlns="http://www.w3.org/2000/svg"
      >
        <path
          d="M5.855 10.08c.108 0 .21-.017.304-.05a.715.715 0 0 0 .264-.173l4.6-4.6a.744.744 0 0 0 .222-.547.778.778 0 0 0-.243-.567.769.769 0 0 0-.568-.223.768.768 0 0
          0-.567.223L5.855 8.155 4.092 6.392a.744.744 0 0 0-.547-.223.778.778 0 0 0-.567.243.769.769 0 0 0-.223.568c0 .23.074.418.223.567l2.31 2.31a.715.715 0 0
          0 .263.173c.095.033.196.05.304.05z"
          fill="var(--text)" style="stroke-width:1"
        />
      </svg>
    </div>
  </div>
</template>

<style scoped>
.checkbox {
  display: flex;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  width: 1rem;
  height: 1rem;
  border: 1px solid var(--text-light);
  background: var(--foreground-light);
  border-radius: 4px;
  /* padding: 2px; */
}
.checkbox.disabled {
  opacity: 0.5;
  pointer-events: none;
}
.inner {
  width: 100%;
  height: 100%;
}
</style>
