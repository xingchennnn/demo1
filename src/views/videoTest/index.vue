<template>
  <div>
    <div class="video-container">
      <div class="video-title">
        <div class="back" @click="router.go(-1)">返回</div><span>视频播放器</span>
      </div>
      <video id="video" class="video-player" controls :src="videoUrl"></video>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue';
import { useRouter } from 'vue-router';

const router = useRouter();


const videoUrl = ref('');
onMounted(() => {
  videoUrl.value = '/mp4/test1.mp4';
});

</script>

<style scoped lang="scss">
.video-player {
  width: 100%;
  height: 90vh;
  /* object-fit:cover; */
  object-fit: contain;

  &:focus {
    outline: none;
  }
}
</style>