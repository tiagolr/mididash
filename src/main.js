import './styles.styl'
import { path } from '@tauri-apps/api';
import { createApp } from "vue";
import { createPinia } from 'pinia';
import App from "./App.vue";
import appStore from './stores/app'
import graphStore from './stores/graph'

const pinia = createPinia()
const app = createApp(App)
app.use(pinia)

// expose stores to all components
app.config.globalProperties.$store = {
  app: appStore(),
  graph: graphStore()
}

window.path = path;
window.store = app.config.globalProperties.$store // expose global stores for dev

app.mount("#app");
