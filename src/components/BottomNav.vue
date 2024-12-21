<template>
  <div class="bottom-nav">
    <button @click="navigateTo('home')">Home</button>
    <button @click="navigateTo('study')">Study</button>
    <button @click="navigateTo('play')">Play</button>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';

const router = useRouter();
function navigateTo(routeName: string) {
  router.push({ name: routeName });
}

</script>

<style scoped>
.bottom-nav {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;
  display: flex;
  justify-content: space-around;
  background-color: #fff;
  /* 或者你想要的任何颜色 */
  border-top: 1px solid #ccc;
  /* 可选的分隔线 */
}

.bottom-nav button {
  flex: 1;
  padding: 10px;
  text-align: center;
  font-size: 16px;
  background-color: transparent;
  /* 或者你想要的任何颜色 */
  border: none;
  cursor: pointer;
}

.bottom-nav button:active {
  background-color: #eee;
  /* 按下时的背景色 */
}
</style>
