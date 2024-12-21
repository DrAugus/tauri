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

                <BannerList v-on:send-img="showshow">

                </BannerList>
            </v-navigation-drawer>

            <v-main>
                <slot></slot>
                <BannerPage v-bind:img="state.aa"></BannerPage>
            </v-main>
        </v-layout>
    </v-card>
</template>


<script lang="ts">
import { defineComponent, reactive } from 'vue';
import BannerList from './BannerList.vue'
import BannerPage from './BannerPage.vue'

export default defineComponent({
    setup() {
        const state = reactive({
            aa: '',
        })
        return {
            state
        }
    },
    components: {
        BannerList,
        BannerPage,
    },
    methods: {
        showshow(v: string) {
            this.state.aa = v;
            console.log("aa", v)
        }
    },
})
</script>