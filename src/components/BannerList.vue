<template>

    <v-list lines="two">

        <v-list-item v-for="(v, i) in WISH.characters" :key="i" :title="v.wishName" :subtitle="charZH(v.wish5star)"
            :prepend-avatar="combineCharImage(v.wish5star)" :value="imgName(v)"
            :active-color="colorCharEle(v.wish5star)" rounded="xl" @click="sendImg(v)">

        </v-list-item>

    </v-list>


</template>

<script setup lang="ts">
import { WISH } from '../script/genshin/wish';
import { ElementColor } from '../script/genshin/utils';
import { CHARACTER } from '../script/genshin/characters';

// import BannerPage from './BannerPage.vue';

const imgName = (value: any) => value.name.concat('_' + value.image + '.jpg').toLowerCase().replace(/ /g, '_')

const combineCharImage = (v: string) => 'https://github.com/DrAugus/data/blob/master/game/genshin/characters/' + v + '.png?raw=true'

const combineWishImage = (v: string) => 'https://github.com/DrAugus/data/blob/master/game/genshin/wish/' + v + '?raw=true'

const colorCharEle = (v: string) => {
    const index_ele = Object.values(CHARACTER)[Object.keys(CHARACTER).indexOf(v)].ele;
    return ElementColor[index_ele]
}

const charZH = (v: string) => Object.values(CHARACTER)[Object.keys(CHARACTER).indexOf(v)].name;

let bannerImg = ''

const showBannerPage = (v: any) => {
    bannerImg = combineWishImage(imgName(v))
}

const emit = defineEmits(['send-img'])

const sendImg = (v: any) => {

    showBannerPage(v)
    emit('send-img', bannerImg)
    console.log("bannerImg", bannerImg)
}



</script>