
<template>

    <v-list lines="two">

        <v-list-item v-for="(v, i) in WISH.characters" :key="imgName(v)" :title="v.name" :subtitle="v.shortName"
            :prepend-avatar="combineCharImage(v.wish5star)" :value="imgName(v)"
            :active-color="colorCharEle(v.wish5star)" rounded="xl">

        </v-list-item>

    </v-list>

    <BannerPage :img="bannerImg"></BannerPage>

</template>

<script setup lang="ts">
import { defineComponent } from 'vue';
import { WISH } from '../script/genshin/wish'
import { replaceAndLow, ElementColor } from '../script/genshin/utils'
import { CHARACTER, Characters } from '../script/genshin/characters'

import BannerPage from './BannerPage.vue';

const imgName = (value: any) => value.name.concat('_' + value.image + '.jpg').toLowerCase().replace(/ /g, '_')

const combineCharImage = (v: string) => 'https://github.com/DrAugus/data/blob/master/game/genshin/characters/' + v + '.png?raw=true'

const combineWishImage = (v: string) => 'https://github.com/DrAugus/data/blob/master/game/genshin/wish/' + v + '?raw=true'

const colorCharEle = (v: string) => {
    const index_ele = Object.values(CHARACTER)[Object.keys(CHARACTER).indexOf(v)].ele;
    return ElementColor[index_ele]
}

let bannerImg = ''

const showBannerPage = (v: string) => {
    bannerImg = combineWishImage(imgName(v))
    console.log(bannerImg)
}

// defineComponent({
//     data(){
//         return {
//             bannerImg :'',
//         }
//     },
//     methods:{
//         showBannerPage(v:string){
//             this.bannerImg = combineWishImage(imgName(v))
//         }
//     },
// })


</script>