
<template>

    <v-list lines="two">

        <v-list-item v-for="(v, i) in WISH.characters" :key="imgName(v)" :title="v.name" :subtitle="v.shortName"
            :prepend-avatar="combineCharImage(v.wish5star)" :value="imgName(v)"
            :active-color="colorCharEle(v.wish5star)" rounded="xl"
    
            ></v-list-item>

            <!-- @click="showBannerPage(combineWishImage(imgName(v.name)))" -->

    </v-list>

    <BannerPage :img="combineCharImage('klee')"></BannerPage>

</template>

<script setup lang="ts">
import { WISH } from '../script/genshin/wish'
import { replaceAndLow, ElementColor } from '../script/genshin/utils'
import { CHARACTER, Characters } from '../script/genshin/characters'

import BannerPage from './BannerPage.vue';

const imgName = (value: any) => value.name.concat('_' + value.image + '.jpg').toLowerCase().replace(/ /g, '_')

const combineCharImage = (v: string) => 'https://github.com/DrAugus/data/blob/master/game/genshin/characters/' + v + '.png?raw=true'

const combineWishImage = (v:string )  => 'https://github.com/DrAugus/data/blob/master/game/genshin/wish/' + v + '?raw=true'

const colorCharEle = (v: string) => {
    const index_ele = Object.values(CHARACTER)[Object.keys(CHARACTER).indexOf(v)].ele;
    return ElementColor[index_ele]
}

// https://vuejs.org/guide/typescript/composition-api.html#typing-component-emits
// type-based
const emit = defineEmits<{
  (e: 'showBannerPage', value: string): {
    
  }
}>()






</script>