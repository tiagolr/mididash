<template>
  <div style="margin-top: 50px; margin-bottom: 50px">
    <h1>Vnodes</h1>

    <h2>
      Canvas <a href="https://github.com/txlabs/vnodes/blob/master/src/demo/Sink.vue" target="_blank">&lt;/&gt;</a>
    </h2>
    <canvas-demo>
    </canvas-demo>

    <h2>
      Sink <a href="https://github.com/txlabs/vnodes/blob/master/src/demo/Sink.vue" target="_blank">&lt;/&gt;</a>
    </h2>
    <sink>
    </sink>

    <!-- <h2>Edit <a href="https://github.com/txlabs/vnodes/blob/master/src/demo/Edit.vue" target="_blank">&lt;/&gt;</a></h2>
    <edit>
    </edit> -->

    <h2>Ports <a href="https://github.com/txlabs/vnodes/blob/master/src/demo/Ports.vue" target="_blank">&lt;/&gt;</a></h2>
    <ports>
    </ports>

    <!-- <h2>Groups <a href="https://github.com/txlabs/vnodes/blob/master/src/demo/Groups.vue" target="_blank">&lt;/&gt;</a></h2>
    <groups>
    </groups> -->

    <h2>Labels <a href="https://github.com/txlabs/vnodes/blob/master/src/demo/Labels.vue" target="_blank">&lt;/&gt;</a></h2>
    <labels>
    </labels>

    <h2>Styles <a href="https://github.com/txlabs/vnodes/blob/master/src/demo/Styles.vue" target="_blank">&lt;/&gt;</a></h2>
    <styles>
    </styles>

    <h2>Markers <a href="https://github.com/txlabs/vnodes/blob/master/src/demo/Markers.vue" target="_blank">&lt;/&gt;</a></h2>
    <markers>
    </markers>

    <h2>Benchpress <a href="https://github.com/txlabs/vnodes/blob/master/src/demo/Benchmark.vue" target="_blank">&lt;/&gt;</a></h2>
    <benchpress>
    </benchpress>
  </div>
</template>

<script>
import Sink from './Sink.vue'
import Edit from './Edit.vue'
import Ports from './Ports.vue'
import Labels from './Labels.vue'
import Benchpress from './Benchmark.vue'
import Styles from './Styles.vue'
import Groups from './Groups.vue'
import Markers from './Markers.vue'
import CanvasDemo from './CanvasDemo.vue'
export default {
  components: {
    Sink,
    Edit,
    Ports,
    Labels,
    Benchpress,
    Styles,
    Groups,
    Markers,
    CanvasDemo
  },
}
</script>

<style scoped>
h2 {
  margin-top: 50px;
  width: calc(100% - 225px);
}
h2 a {
  color: #4a4a4a;
  font-size: 0.75rem;
  float: right
}
</style>

<style>
.screen {
  background-color: white;
}
h2 {
  font-size: 1.5rem;
}
body {
  background-color: #f9f9f9;
  max-width: 56rem;
}
.demo {
  display: flex;
}
.viewport {
  height: 500px;
  flex-grow: 1;
}
.sidebar {
  padding-left: 20px;
  width: 200px;
  max-width: 200px;
  flex-shrink: 0;
}

@media only screen and (max-width: 1000px) {
  .demo {
    flex-wrap: wrap;
  }
  .viewport {
    width: 100%;
  }
  .sidebar {
    padding-left: 0;
    display: flex;
    flex-wrap: wrap;
    min-width: calc(100%);
    gap: 10px;
    margin-top: 20px;
  }
  h2 {
    width: 100% !important;
  }
}
</style>
