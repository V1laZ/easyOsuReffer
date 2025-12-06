import { createApp } from 'vue'
import App from './App.vue'
import { createWebHashHistory, createRouter } from 'vue-router'

import './assets/css/main.css'

import LoginView from './pages/LoginView.vue'
import ChatView from './pages/ChatView.vue'
import { PendingRequest } from '@/types'

export const dialogRegistry = new Map<string, HTMLDialogElement>()
export const avatarCache = new Map<string, string>()
export const pendingRequests = new Map<string, PendingRequest>()

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
