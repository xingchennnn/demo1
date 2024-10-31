import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { resolve } from "path";

// 获取环境变量
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

  // vite 选项 针对 Tauri 开发环境和仅在 `tauri dev` 或 `tauri build` 时应用的选项
  // 1. 防止vite掩盖rust错误
  clearScreen: false,
  // 2. tauri需要一个固定端口，如果该端口不可用，则失败
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. 告诉vite不要监视tauri的源码
      ignored: ["**/src-tauri/**"],
    },
  },
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
    },
  },
}));
