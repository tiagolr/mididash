<script>
export default {
  props: {
    checked: Boolean,
    color: Boolean,
    noText: String,
    yesText: String,
    disabled: Boolean,
    small: Boolean,
  },
  emits: [
    'click'
  ],
}
</script>

<template>
  <div class="container" :class="{checked, color, disabled, small}" @click="!disabled && $emit('click', $event)">
    <span v-show="!checked" class="label no">{{ noText || 'No' }}</span>
    <div class="inner">
    </div>
    <span v-show="checked" class="label yes">{{ yesText || 'Yes' }}</span>
  </div>
</template>

<style lang="scss" scoped>
* {
  transition: all 0.3s;
}
.label {
  color: #fefefe;
  font-size: 10px;
  text-transform: uppercase;
  font-weight: bold;
  position: absolute;
  top: 9px;
  user-select: none;
}
.small .label {
  top: 7px;
}
.small .label.yes {
  left: 12px;
}
.label.yes {
  left: 14px;
}
.small .label.no {
  right: 12px;
}
.label.no {
  right: 14px;
}
.container {
  position: relative;
  width: 68px;
  height: 32px;
  background: #090909;
  border-radius: 100px;
  cursor: pointer;
}
.container.small {
  width: 60px;
  height: 24px;
}
.checked.container {
  background: #090909;
}
.color.container {
  background: var(--error);
}
.checked.color.container {
  background: var(--success);
}
.container.disabled {
  background: #DFDFDF !important;
  .label {
    color: #DFDFDF !important;
  }
}
.inner {
  position: absolute;
  width: 24px;
  height: 24px;
  background: #FCFCFC;
  top: calc(50% - 24px/2);
  left: 5px;
  border-radius: 50%;
}

.small .inner {
  width: 18px;
  height: 18px;
  top: calc(50% - 18px/2);
  left: 4px;
}
.small.checked .inner {
  left: calc(100% - 18px - 4px);
}
.checked .inner {
  left: calc(100% - 24px - 5px)
}
</style>
