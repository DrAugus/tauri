<template>
    <HomeSider>
        <template v-slot>
            <img alt="logo" width="160" src="../assets/logo.jpg" />
            <HelloWorld msg="AUGUS" />
        </template>
    </HomeSider>
</template>

<script setup lang="ts">
import HelloWorld from './HelloWorld.vue'
import HomeSider from './HomeSider.vue';
</script>