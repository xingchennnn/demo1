<template>
  <div>
    <button  @click="handleClick">点击</button>
  </div>
</template>

<script setup lang="ts">
import { loginWin } from '../../windows/actions';
import CreateMsgBox from './msg';


const handleClick = () => { 
  console.log('点击了按钮')
  CreateMsgBox()
  loginWin()
}

</script>

<style lang="scss" scoped>

</style>