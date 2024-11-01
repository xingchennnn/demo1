<template>
  <div>
      <router-view>
      </router-view>
  </div>
</template>

<script setup lang="ts">
import CreateTraymenu from '@/views/menu/index'
import { onMounted } from 'vue';


onMounted(() => {
  CreateTraymenu()
})
</script>

<style scoped>

</style>