<template>
  <div>
    <button @click="handleClick">点击闪烁托盘图标</button>
    <button @click="flashTray(false)">取消闪烁托盘图标</button>
    <button @click="handleNotifyClick">发送消息通知</button>
    <button @click="openAboutWin">about</button>
    <button @click="sleep">睡眠</button>
    <button @click="goTestVideo">testVideo</button>
    <button @click="goWindowProxy">windowProxy</button>
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

const router = useRouter();

const handleClick = () => {
  console.log('点击了按钮')
  // CreateMsgBox()
  // loginWin()
  flashTray(true)
}


/**
 * 封装设置托盘图标闪烁 flashTray(true) 和取消闪烁 flashTray(false) 
 */
const flashTimer = ref()
const flashTray = async (bool: boolean) => {
  let flag = true
  if (bool) {
    TrayIcon.getById('tray').then(async (res: TrayIcon | null) => {
      clearInterval(flashTimer.value)
      flashTimer.value = setInterval(() => {
        if (flag) {
          // 关闭托盘图标
          res?.setIcon(null)
          
        } else {
          // 支持把自定义图标放在默认icons文件夹，通过如下方式设置图标
          // res.setIcon('icons/msg.png')
          // 支持把自定义图标放在自定义文件夹tray，需要配置tauri.conf.json参数 "bundle": {"resources": ["tray"]} ,
          //如果是png格式的图标，需要在cargo.toml中配置 tauri = { version = "2.0.0", features = [ "tray-icon", "image-png" ] }
          res?.setIcon('tray/msg.png')
        }
        flag = !flag
      }, 500)
    })
  } else {
    clearInterval(flashTimer.value)
    let tray: TrayIcon | null = await TrayIcon.getById("tray")
    let option = {
      id: "tray",
      title: "托盘图标",
      icon:'tray/icoc.png'
    }
    tray?.setIcon(option.icon)
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

const goTestVideo = () => {
  // console.log('打开测试视频窗口')
  router.push('/testVideo')
}


//
const goWindowProxy = () => {
  router.push('/windowProxy')
}

</script>

<style lang="scss" scoped></style>