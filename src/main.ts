import { createApp } from "vue";
import App from "./App.vue";
import { createWebHashHistory, createRouter } from 'vue-router'

import "./assets/css/main.css";

import LoginView from "./pages/LoginView.vue";
import ChatView from "./pages/ChatView.vue";


const routes = [
  { path: '/login', component: LoginView },
  { path: '/chat', component: ChatView },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

createApp(App)
  .use(router)
  .mount("#app");
