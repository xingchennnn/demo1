<template>
  <div>
    <h1>About</h1>
    <button @click="router.back()">返回</button>
    <button @click="openConsole()">打开控制台</button>
    <img :src="pic" alt="">
  </div>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

const pic = ref()

const openConsole =async () => {
  console.log('打开控制台' ,window );
  await invoke('open_devtools');
  pic.value = 'https://www.baidu.com/img/PCtm_d9c8750bed0b3c7d089fa7d55720d6cf.png';
};

const router = useRouter();
</script>

<style scoped>

</style>