import { createApp } from 'vue'
import App from './App.vue'
import { createWebHashHistory, createRouter } from 'vue-router'

import './assets/css/main.css'

import LoginView from './pages/LoginView.vue'
import ChatView from './pages/ChatView.vue'

export const dialogRegistry = new Map<string, HTMLDialogElement>()

const routes = [
  { path: '/login', component: LoginView },
  { path: '/', component: ChatView },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

createApp(App)
  .use(router)
  .mount('#app')
