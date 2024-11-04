<template>
  <div>
    <el-button @click="router.back()" link>返回</el-button>
    windowProxy
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { useRouter } from 'vue-router';

const router = useRouter();


onMounted(() => {
  console.log('windowProxy mounted')
})

</script>

<style lang="scss" scoped>

</style>