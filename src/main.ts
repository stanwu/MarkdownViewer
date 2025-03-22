// 引入全局樣式表
import "./styles.css";
// 引入 Svelte 主應用程式元件
import App from "./App.svelte";

// 初始化 Svelte 應用程式
// 將應用程式掛載到 id 為 "app" 的 DOM 元素上
const app = new App({
  target: document.getElementById("app"),
});

// 匯出應用程式實例，以便其他模組可以引入
export default app;
