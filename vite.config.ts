// 引入 Vite 配置函數
import { defineConfig } from "vite";
// 引入 Svelte 的 Vite 插件
import { svelte } from "@sveltejs/vite-plugin-svelte";
// 引入 Svelte 預處理器，用於支援 TypeScript 等功能
import sveltePreprocess from "svelte-preprocess";

// Vite 配置文件 - https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    svelte({
      // 設定 Svelte 預處理器
      preprocess: [
        sveltePreprocess({
          // 啟用 TypeScript 支援
          typescript: true,
        }),
      ],
    }),
  ],
  // 針對 Tauri 開發量身定製的 Vite 選項，僅在 `tauri dev` 或 `tauri build` 時套用
  
  // 防止 Vite 隱藏 Rust 錯誤訊息
  clearScreen: false,
  
  // Tauri 需要一個固定的端口，如果該端口不可用則會失敗
  server: {
    port: 1420,
    strictPort: true,
  },
  
  // 支援使用 `TAURI_DEBUG` 和其他環境變數
  // https://tauri.studio/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
  
  build: {
    // Tauri 支援 ES2021
    target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
    
    // 在偵錯建置中不進行壓縮
    minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
    
    // 為偵錯建置產生源碼映射
    sourcemap: !!process.env.TAURI_DEBUG,
  },
});
