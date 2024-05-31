<script setup lang="ts">
import { ref } from 'vue'

defineProps<{ msg: string }>()

const count = ref(0)
</script>

<template>
  <h1>{{ msg }}</h1>

  <a href="https://tauri.app/v1/guides/getting-started/setup/vite">TAURI GUIDE</a>

  <p>
    <a href="https://draugus.github.io/">Game Site</a> <br />
    <a href="https://augusmeow.github.io/">Augus Homepage</a>
  </p>

  <p>第一个多平台软件</p>

  <p>当前处于测试开发期...摸鱼...咕咕咕</p>


</template>

<style scoped>
a {
  color: #42b983;
}

label {
  margin: 0 0.5em;
  font-weight: bold;
}

code {
  background-color: #eee;
  padding: 2px 4px;
  border-radius: 4px;
  color: #304455;
}
</style>
