<template>

    <v-img class="bg-white" height="300" :aspect-ratio="1" :src="props.img"></v-img>

    link | {{ props.img }}

</template>

<script setup lang="ts">
const props = defineProps<{
    img: string
}>()

console.log("img", props.img)
</script>