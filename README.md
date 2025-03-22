# Markdown 檢視器

一個簡單獨立的、僅用於檢視 markdown 的 Windows 11 應用程式。

使用 [Tauri](https://tauri.app/) 建構 — [Rust](https://www.rust-lang.org) + [SvelteKit](https://kit.svelte.dev/) + [TypeScript](https://www.typescriptlang.org/)。

採用由 [sindresorhus](https://github.com/sindresorhus/generate-github-markdown-css) 製作的 GitHub 風格 markdown 樣式，並使用 [comrak](https://github.com/kivikakk/comrak) 進行渲染。使用 [window-vibrancy](https://github.com/tauri-apps/window-vibrancy) 為視窗添加 Mica 效果。

## 使用方法

- 從[發布頁面](https://github.com/alecames/MarkdownViewer/releases/latest)下載最新的 `.exe` 可執行檔或 `.msi` 安裝程式
- 在 markdown 檔案上按右鍵，選擇「開啟方式」，然後選擇已下載或安裝的可執行檔
- [選擇性] 將該可執行檔設為開啟 `.md` 檔案的預設程式

或者，您也可以從原始碼安裝：

- 複製此儲存庫
- 執行 `npm install` 安裝相依套件
- 執行 `npm run tauri build` 以建立 `.exe` 可執行檔和 `.msi` 安裝包
- 重複上述步驟，將可執行檔設為開啟 `.md` 檔案的預設程式

## 截圖

![此說明文件的截圖](pics/image.png)
![此說明文件在淺色模式下的截圖](pics/image2.png)

## 已知問題

- 未在 Windows 10 及更早版本測試，視窗背景可能無法如預期顯示
- 相對路徑的圖片嵌入無法運作
- YouTube/影片嵌入無法運作

## 待辦事項

- [ ] 修復相對路徑的圖片嵌入問題
- [ ] 添加在應用程式中切換深色/淺色模式的選項
- [ ] 添加快捷方式以在預設文字編輯器中編輯
- [ ] 添加切換 markdown 渲染的選項
- [ ] 為程式碼區塊添加語法高亮
- [ ] 在 Windows 安裝程式中添加 `.md` 檔案的檔案關聯選項
- [ ] 調整 Windows 安裝程式以預設防止桌面快捷方式
