<template>

    <v-img class="bg-white" height="300" :aspect-ratio="1" :src="props.img"></v-img>

    <br>

    <v-btn color="#f28413" prepend-icon="mdi-cloud-upload">
        160
    </v-btn>
    {{ }}
    <v-btn color="#f28413" prepend-icon="mdi-cloud-upload">
        1600
    </v-btn>

    <br>
    todo btn page

</template>

<script setup lang="ts">
const props = defineProps<{
    img: string
}>()

const shows = props.img
console.log(shows)

console.log("img", props.img)
</script>