<template>
  <div>
    <button @click="handleClick">点击闪烁托盘图标</button>
    <button @click="flashTray(false)">取消闪烁托盘图标</button>
    <button @click="handleNotifyClick">发送消息通知</button>
    <button @click="openAboutWin">about</button>
    <button @click="sleep">睡眠</button>
    <img :src="png" alt="" style="width: 150px; height: 150px;border: 1px solid #ccc;">
  </div>
</template>

<script setup lang="ts">
// import { loginWin } from '@/windows/actions';
// import CreateMsgBox from './msg';
import Notification from "@/windows/notification"
import { useRouter } from 'vue-router';
import { ref } from 'vue';
import { TrayIcon } from '@tauri-apps/api/tray';
import { invoke } from '@tauri-apps/api/core';
import ping from "@/assets/vue.svg"

const router = useRouter();

const handleClick = () => {
  console.log('点击了按钮')
  // CreateMsgBox()
  // loginWin()
  flashTray(true)
}


const png = ref()
/**
 * 封装设置托盘图标闪烁 flashTray(true) 和取消闪烁 flashTray(false) 
 */
const flashTimer = ref()
const flashTray = async (bool: boolean) => {
  let flag = true
  if (bool) {
    TrayIcon.getById('tray').then(async (res) => {
      clearInterval(flashTimer.value)
      flashTimer.value = setInterval(() => {
        if (flag) {
          // 关闭托盘图标
          res?.setIcon(null)
          
        } else {
          // 支持把自定义图标放在默认icons文件夹，通过如下方式设置图标
          // res.setIcon('icons/msg.png')
          // 支持把自定义图标放在自定义文件夹tray，需要配置tauri.conf.json参数 "bundle": {"resources": ["tray"]}

          res?.setIcon('tray/msg.png')
        }
        flag = !flag
      }, 500)
    })
  } else {
    clearInterval(flashTimer.value)
    // let tray = await TrayIcon.getById("tray")
    let option = {
      id: "tray",
      title: "托盘图标",
      icon:'./../../assets/vue.svg'
    }
    let tray2 = await TrayIcon.new(option)
    tray2?.setIcon(option.icon)
    png.value = option.icon

  }
}





const handleNotifyClick = () => {
  console.log('发送消息通知')
  Notification.send('通知标题', '通知内容')
}

const openAboutWin = () => {
  console.log('打开关于窗口')
  router.push('/about')
}

const sleep =async () => {
  console.log('睡眠')
  // 睡眠
  await invoke('close',{close:'app'})
}

</script>

<style lang="scss" scoped></style>