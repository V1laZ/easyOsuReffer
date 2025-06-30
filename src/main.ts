import { createApp } from "vue";
import App from "./App.vue";
import { createWebHashHistory, createRouter } from 'vue-router'

import "./assets/css/main.css";

import LoginView from "./pages/LoginView.vue";
import HomeView from "./pages/HomeView.vue";


const routes = [
  { path: '/', component: HomeView },
  { path: '/login', component: LoginView },
]

const router = createRouter({
  history: createWebHashHistory(),
  routes,
})

createApp(App)
  .use(router)
  .mount("#app");
