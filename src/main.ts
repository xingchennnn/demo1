import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { createPinia } from 'pinia'
import "./assets/style/main.css";
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import zhCn from 'element-plus/es/locale/lang/zh-cn'
//导入dayjs的区域设置
import 'dayjs/locale/zh-cn'

const pinia = createPinia()

createApp(App)
.use(router)
  .use(pinia)
  .use(ElementPlus , { locale: zhCn })
.mount("#app");
