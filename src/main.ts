import { createApp } from "vue";
// https://router.vuejs.org/zh/
import { createMemoryHistory, createRouter } from 'vue-router';
import App from "./App.vue";

import Home from './components/Home.vue';
import Study from './components/Study.vue';
import Play from './components/Play.vue';

const routes = [
    { path: '/', name: "home", component: Home },
    { path: '/study', name: "study", component: Study },
    { path: '/play', name: "play", component: Play },
]

const router = createRouter({
    history: createMemoryHistory(),
    routes,
})

createApp(App)
    .use(router).mount("#app");
