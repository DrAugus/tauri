<template>
    <v-img class="bg-white" width="300" :aspect-ratio="1" :src="img" cover></v-img>
</template>

<script setup lang="ts">
defineProps({
    img: {
        type: String,
        default: ''
    },
})
</script>