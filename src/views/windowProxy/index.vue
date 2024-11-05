<template>
  <div>
    <el-button @click="router.back()" link>返回</el-button>
    windowProxy
    <el-button @click="startTest2()" disabled>测试2</el-button>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
const router = useRouter();


onMounted(() => {
  console.log('windowProxy mounted')
})

const startTest2 = async () => { 
 const result = await invoke('start_proxy')
  console.log(result)
}
</script>

<style lang="scss" scoped>

</style>