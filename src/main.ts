import { createApp } from 'vue'
import App from './App.vue'
import { createWebHashHistory, createRouter } from 'vue-router'

import './assets/css/main.css'

import LoginView from './pages/LoginView.vue'
import ChatView from './pages/ChatView.vue'
import MappoolsView from './pages/MappoolsView.vue'
export const avatarCache = new Map<string, string>()

declare module 'vue-router' {
  interface RouteMeta {
    order?: number
    transition?: string
  }
}

const routes = [
  { path: '/login', component: LoginView, meta: { order: 0 } },
  { path: '/', component: ChatView, meta: { order: 1 } },
  { path: '/mappools', component: MappoolsView, meta: { order: 2 } },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

router.afterEach((to, from) => {
  const toOrder = to.meta.order ?? 0
  const fromOrder = from.meta.order ?? 0
  to.meta.transition = toOrder < fromOrder ? 'slide-right' : 'slide-left'
})

createApp(App)
  .use(router)
  .mount('#app')
