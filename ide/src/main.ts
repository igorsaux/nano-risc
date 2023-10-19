import './assets/index.css'

import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'

async function main() {
  const pinia = createPinia()
  const app = createApp(App)

  app.use(pinia)
  app.mount('body')
}

main()
