# Markdown 檢視器專案文件說明

本文件提供 Markdown 檢視器專案中所有檔案的概述和說明。

## 專案結構

| 路徑 | 檔案名稱 | 說明 |
|------|---------|------|
| / | .gitignore | 定義 Git 版本控制中要忽略的檔案和目錄 |
| / | index.html | 應用程式的主要 HTML 入口點 |
| / | LICENSE | 專案的授權條款 |
| / | package.json | 專案依賴項和腳本的設定檔 |
| / | README.md | 專案的主要說明文件 |
| / | tauri.code-workspace | Visual Studio Code 的工作區設定 |
| / | tsconfig.json | TypeScript 編譯器設定 |
| / | tsconfig.node.json | Node.js 環境下的 TypeScript 設定 |
| / | vite.config.ts | Vite 前端建置工具的設定檔 |
| /pics/ | image.png | 專案截圖（深色模式） |
| /pics/ | image2.png | 專案截圖（淺色模式） |
| /public/ | svelte.svg | Svelte 框架的圖示 |
| /public/ | tauri.svg | Tauri 框架的圖示 |
| /public/ | vite.svg | Vite 工具的圖示 |
| /src/ | App.svelte | 主要的 Svelte 應用元件 |
| /src/ | main.ts | 應用程式的 TypeScript 入口點 |
| /src/ | styles.css | 全域 CSS 樣式 |
| /src/ | vite-env.d.ts | Vite 的 TypeScript 型別定義 |
| /src/lib/ | MarkdownViewer.svelte | Markdown 檢視元件（應用核心） |
| /src-tauri/ | .gitignore | Tauri 專案的 Git 忽略檔 |
| /src-tauri/ | build.rs | Tauri 構建時執行的 Rust 腳本 |
| /src-tauri/ | Cargo.lock | Rust 依賴項的版本鎖定檔 |
| /src-tauri/ | Cargo.toml | Rust 專案配置和依賴定義 |
| /src-tauri/ | tauri.conf.json | Tauri 應用程式的配置檔 |
| /src-tauri/icons/ | 128x128.png | 應用圖示（128x128 像素） |
| /src-tauri/icons/ | 128x128@2x.png | 高解析度應用圖示（256x256 像素） |
| /src-tauri/icons/ | 32x32.png | 應用圖示（32x32 像素） |
| /src-tauri/icons/ | favicon.svg | 網頁標籤圖示 |
| /src-tauri/icons/ | icon.ico | Windows 應用圖示 |
| /src-tauri/icons/ | icon.png | 通用應用圖示 |
| /src-tauri/src/ | main.rs | Tauri 後端的 Rust 主程式（核心功能） |

## 核心檔案說明

### 前端（Svelte/TypeScript）

**MarkdownViewer.svelte** - 提供 Markdown 檢視功能的主要元件，包含：
- 透過 Tauri API 與 Rust 後端通訊
- 接收並顯示 Markdown 轉換後的 HTML
- 處理連結攔截和外部瀏覽器開啟
- 提供在編輯器中開啟文檔的功能

### 後端（Rust/Tauri）

**main.rs** - Rust 後端的核心邏輯：
- 使用 comrak 庫解析和轉換 Markdown 為 HTML
- 實現檔案讀取和路徑管理
- 處理命令列參數
- 設定視窗效果（如 Windows 上的 Mica 效果）
- 配置 Tauri 應用程式和視窗

## 技術棧

- **前端**：Svelte、TypeScript
- **後端**：Rust、Tauri
- **Markdown 轉換**：comrak
- **視窗效果**：window-vibrancy
- **構建工具**：Vite
