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
                <BannerPage v-bind:img="aa"></BannerPage>
            </v-main>
        </v-layout>
    </v-card>
</template>


<script setup lang="ts">
import BannerList from './BannerList.vue'
import BannerPage from './BannerPage.vue'
import HelloWorld from './HelloWorld.vue';

let aa = 'https://github.com/DrAugus/data/blob/master/game/genshin/wish/dance_of_lanterns_1.jpg?raw=true'
// aa = ''

const showshow = (v: string) => {
    aa = v;
    console.log("aa", aa)
}


</script>