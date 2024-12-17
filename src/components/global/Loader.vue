<script>
export default {
  props: {
    color: {
      type: String,
      default: ''
    },
    size: {
      type: Number,
      default: 3
    },
    style: {
      type: String,
      default: ''
    }
  }
}
</script>

<!-- https://projects.lukehaas.me/css-loaders/ -->
<template>
  <div :style="`position: absolute; ${style}`">
    <div
      class="loader"
      :style="`border-left: 1.1em solid ${color || 'var(--font-color)'};font-size: ${size}px`"
    >
    </div>
  </div>
</template>

<style scoped>
.loader,
.loader:after {
  border-radius: 50%;
  width: 10em;
  height: 10em;
}
.loader {
  margin: 60px auto;
  /* font-size: 3px; */
  position: relative;
  text-indent: -9999em;
  border-top: 1.1em solid rgba(255, 255, 255, 0.2);
  border-right: 1.1em solid rgba(255, 255, 255, 0.2);
  border-bottom: 1.1em solid rgba(255, 255, 255, 0.2);
  /* border-left: 1.1em solid #ffffff; */
  -webkit-transform: translateZ(0);
  -ms-transform: translateZ(0);
  transform: translateZ(0);
  -webkit-animation: load8 1.1s infinite linear;
  animation: load8 1.1s infinite linear;
}
@-webkit-keyframes load8 {
  0% {
    -webkit-transform: rotate(0deg);
    transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
    transform: rotate(360deg);
  }
}
@keyframes load8 {
  0% {
    -webkit-transform: rotate(0deg);
    transform: rotate(0deg);
  }
  100% {
    -webkit-transform: rotate(360deg);
    transform: rotate(360deg);
  }
}
</style>