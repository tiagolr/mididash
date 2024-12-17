<script>
import IInput from '../../src/assets/input.svg'
import IOutput from '../../src/assets/output.svg'
import IVirtual from '../../src/assets/virtual.svg'
import IMap from '../../src/assets/map.svg'
import ISplit from '../../src/assets/split.svg'
import IDelay from '../../src/assets/delay.svg'
import IMonitor from '../../src/assets/monitor.svg'
import INote from '../../src/assets/note.svg'
import IScript from '../../src/assets/script.svg'
import ITrigger from '../../src/assets/piano.svg'
export default {
  components: {
    IInput,
    IOutput,
    IVirtual,
    IMap,
    ISplit,
    IDelay,
    IMonitor,
    INote,
    IScript,
    ITrigger,
  }
}
</script>

<template>
  <div class="app">
    <div class="grid"></div>
    <div class="container">
      <div class="flex justify-center">
        <img src="/logo.svg" alt="" class="logo">
      </div>
      <h1 class="text-center">
        Mididash
      </h1>
      <div style="margin: 1rem 0; display: flex; justify-content: center; gap: 8px">
        <img src="https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white" alt="">
        <img src="https://img.shields.io/badge/Ubuntu-E95420?style=for-the-badge&logo=ubuntu&logoColor=white" alt="">
        <img src="https://img.shields.io/badge/MACOS-adb8c5?style=for-the-badge&logo=macos&logoColor=white" alt="">
      </div>
      <div style="margin: 1rem 0; display: flex; justify-content: center; gap: 8px">
        <img src="https://img.shields.io/github/package-json/v/ruralad/notedown?color=%40&label=latest" alt="">
        <img src="https://img.shields.io/github/issues-raw/ruralad/notedown" alt="">
        <img src="https://img.shields.io/github/license/tiagolr/vnodes">
      </div>
      <div style="margin: 1rem 0; display: flex; justify-content: center; gap: 8px">
        <img src="https://img.shields.io/github/downloads/ruralad/notedown/total" alt="">
      </div>
      <hr>
      <p>
        Mididash is an open source MIDI routing software with a node-based dashboard.
        A modern take on programs like <a href="http://midiox.com" target="_blank">MIDI-OX</a>.
      </p>
      <img src="/ss.png" alt="" style="width: 100%">
      <h3>
        Features
      </h3>
      <ul>
        <li><strong>Cross-platform</strong> available on Windows, Linux and macOS</li>
        <li><strong>Node-based MIDI routing</strong> allows for versatile configurations</li>
        <li><strong>Hot-plugging</strong> reconnect or replace MIDI devices</li>
        <li><strong>MIDI monitoring</strong> for inputs, outputs or individual nodes</li>
        <li><strong>Pre-configured nodes</strong> for MIDI splitting, mapping and more</li>
        <li><strong>Script nodes</strong> using Lua 5.4</li>
        <li><strong>Lightweight</strong> installer under 10MB</li>
        <li><strong>High performance</strong> built with Tauri and Rust</li>
      </ul>
      <h3>
        Download
      </h3>
      <p>
        The latest version and others can be found on <a href="">Github releases</a>
      </p>
      <h3>Beta software</h3>
      <p>
        This software is under active development, the core features are ready but there may be changes in appearance, stability patches and other changes until version 1.0.
      </p>
      <h3>How does it compare with MIDI-OX?</h3>
      <p>
        MIDI-OX is the main reference for this program, this one offers more routing capabilities and practical scripting via Lua nodes,
        it does not contain the same amount of MIDI tools like bank selectors or NPRN calculators but those can be added over time.
      </p>
      <p>
        In terms of performance, it's about the same CPU usage (e.g. 5% CPU on high midi throughput),
        the bundle size is small but cannot compete with 2MB of MIDI-OX,
        the memory footprint of the program is also similar but the webview can consume a few dozen MBs.
      </p>
      <h3>
        Nodes
      </h3>
      <p>Nodes can be dragged to the main viewport and connected to any other nodes allowing for flexible configurations.</p>
      <div class="node">
        <i-input class="icon"></i-input>
        <div><strong>Input</strong> MIDI device</div>
      </div>
      <div class="node">
        <i-output class="icon"></i-output>
        <div><strong>Output</strong> MIDI device</div>
      </div>
      <div class="node">
        <i-virtual class="icon"></i-virtual>
        <div><strong>Virtual</strong> cable with MIDI through (Linux and macOS only)</div>
      </div>
      <div class="node">
        <i-split class="icon"></i-split>
        <div><strong>Split</strong> MIDI signal into different ports depending on status byte</div>
      </div>
      <div class="node">
        <i-map class="icon"></i-map>
        <div><strong>Map</strong> the incoming MIDI message to another message</div>
      </div>
      <div class="node">
        <i-delay class="icon"></i-delay>
        <div><strong>Delay</strong> the signal from 1 to 60000 milliseconds</div>
      </div>
      <div class="node">
        <i-monitor class="icon"></i-monitor>
        <div><strong>Monitor</strong> signals from any node output port</div>
      </div>
      <div class="node">
        <i-trigger class="icon"></i-trigger>
        <div><strong>Trigger</strong> MIDI notes or custom bytes messages like Sysex</div>
      </div>
      <div class="node">
        <i-note class="icon"></i-note>
        <div><strong>Note</strong>s placeholder</div>
      </div>
      <div class="node">
        <i-script class="icon"></i-script>
        <div><strong>Script</strong> MIDI processing using Lua 5.4</div>
      </div>
      <h3>
        Hot plugging
      </h3>
      <p>
        New devices or disconnected devices are detected every few seconds. To add a new device to the project simply drag and drop it from the sidebar into the viewport.
        To replace a missing device or a device with changed ID click its node (if it exists) and then <code>Replace device</code> from the sidebar
        - it will assign the device to that node and re-establish all previous connections.
      </p>
      <h3>
        Scripts and templates
      </h3>
      <p>
        Using Lua 5.4 it's possible to program nodes for any kind MIDI processing. A few examples have been included to demonstrate how to modify and forward bytes.
        To test a script press <code>Ctrl+Enter</code> or <code>Cmd+Enter</code> from the code editor. Once a script is loaded into a node,
        it will run every time a signal is received on its input port.
      </p>
      <p>
        Scripts can be saved as global templates to be reused in projects, to do so click the settings icon near the code editor and then <code>Save as template</code>.
      </p>
      <h3>License</h3>
      <p>
        <a href="https://www.gnu.org/licenses/gpl-3.0.en.html" target="_blank">GPL 3</a>
      </p>
      <div class="text-center" style="color: #7be0ff;opacity: 0.5;">
        -- tilr © 2025 --
      </div>
    </div>
  </div>
</template>

<style>
body {
  background: var(--foreground-dark);
  font-family: apple-system, BlinkMacSystemFont, "Segoe UI", "Noto Sans", Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji";
}
* {
  line-height: 1.5;
  font-size: 16px;
}
a {
  color: #7be0ff;
}
body {
  overflow: auto;
  user-select: auto;
}
h1 {
  font-size: 2rem;
  margin: 1rem 0;
}
h3 {
  margin-top: 1.5rem;
  margin-bottom: 1rem;
  font-size: 1.5rem;
}
p {
  margin-bottom: 1rem;
}
hr {
  background: none;
  color: none;
  border: none;
  border-bottom: 1px solid #7be0ff;
  margin: 1rem 0;
}
li:not(:first-child) {
  margin-top: 0.25rem
}
code {
  padding: .2em .4em;
  margin: 0;
  font-size: 85%;
  white-space: break-spaces;
  background-color: #7be0ff;
  color: var(--background);
  border-radius: 6px;
}
</style>

<style scoped>
.app {
  overflow: auto;
}
.grid {
  background-image: url('/grid.svg');
  background-repeat: repeat;
  position: fixed;
  width: 100vw;
  height: 100vh;
  z-index: -1;
}
.container {
  max-width: 900px;
  padding: 32px;
  background: #232929;
  margin: auto;
  min-height: 100vh;
  color: var(--copy);
}
.logo {
  margin: auto;
  width: 128px;
  height: 128px;
  padding: 8px;
  border-radius: 8px;
  /* margin-bottom: 8px; */
}
.node {
  display: flex;
  align-items: center;
  gap: 8px;
}
.node:not(:last-child) {
  margin-bottom: 0.5rem;
}
.node .icon {
  width: 32px;
  height: 32px;
  padding: 4px;
  border-radius: 8px;
  /* background: var(--background); */
}
</style>
