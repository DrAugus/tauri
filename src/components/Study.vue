<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");


async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>

    <h1>Welcome to Tauri + Vue</h1>

    <div class="row">
        <a href="https://vitejs.dev" target="_blank">
            <img src="/vite.svg" class="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank">
            <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://vuejs.org/" target="_blank">
            <img src="/vue.svg" class="logo vue" alt="Vue logo" />
        </a>
        <a href="https://www.rust-lang.org/" target="_blank">
            <img src="/rust.svg" class="logo rust" alt="Rust logo" />
        </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more...</p>

    <form class="row" @submit.prevent="greet">
        <input id="greet-input" v-model="name" placeholder="Enter your name..." />
        <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>

</template>

<style scoped>
.logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
    filter: drop-shadow(0 0 2em #249b73);
}

.logo.rust:hover {
    filter: drop-shadow(0 0 2em #222422);
}
</style>
