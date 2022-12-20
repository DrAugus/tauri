<template>
    <v-card>
        <v-layout>
            <v-navigation-drawer theme="auto" rail permanent>
                <v-list-item nav prepend-avatar="../assets/logo.jpg"></v-list-item>

                <v-divider></v-divider>

                <v-list density="compact" nav>
                    <v-list-item prepend-icon="mdi-view-dashboard" value="dashboard"></v-list-item>

                    <v-list-item prepend-icon="mdi-forum" value="messages"></v-list-item>
                </v-list>
            </v-navigation-drawer>

            <v-navigation-drawer permanent>

                <BannerList>

                </BannerList>
            </v-navigation-drawer>

            <v-main>
                <slot></slot>
            </v-main>
        </v-layout>
    </v-card>
</template>


<script setup lang="ts">

import BannerList from './BannerList.vue'
import BannerPage from './BannerPage.vue'
import HelloWorld from './HelloWorld.vue';
defineProps({
    icon: {
        type: String,
        default: ''
    },
    iconClass: {
        type: String,
        default: ''
    },
    blurry: {
        type: Boolean,
        default: true,
        required: false
    }
})
</script>