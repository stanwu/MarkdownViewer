// 如果不是在除錯模式下，則將 Windows 子系統設為 "windows"
// 這會隱藏命令視窗，只顯示 GUI 介面
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 引入需要的依賴項
use comrak::{markdown_to_html, ComrakExtensionOptions, ComrakOptions}; // Markdown 解析器
use std::{fs, sync::Arc}; // 檔案系統和 Arc (原子參考計數) 模組
use tauri::Manager; // Tauri 視窗管理功能
use window_vibrancy::apply_mica; // Windows Mica 視窗效果

// Tauri 命令：開啟並解析 Markdown 檔案
// 此函數接收 Markdown 檔案路徑，並將內容轉換為 HTML
#[tauri::command]
fn open_markdown(path: String) -> Result<String, String> {
    // 讀取檔案內容，如果失敗則返回錯誤
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    
    // 設定 Comrak Markdown 解析選項
    // 啟用各種 Markdown 擴展功能
    let options = ComrakOptions {
        extension: ComrakExtensionOptions {
            strikethrough: true,      // 啟用刪除線
            table: true,              // 啟用表格
            autolink: true,           // 自動偵測連結
            tasklist: true,           // 啟用任務清單
            superscript: true,        // 啟用上標
            footnotes: true,          // 啟用腳註
            description_lists: true,  // 啟用描述清單
            ..ComrakExtensionOptions::default()
        },
        ..ComrakOptions::default()
    };
    
    // 將 Markdown 轉換為 HTML
    let html_output = markdown_to_html(&content, &options);
    
    // 返回 HTML 內容
    Ok(html_output)
}

// Tauri 命令：獲取命令列中傳遞的 Markdown 檔案路徑
#[tauri::command]
fn send_markdown_path() -> Result<String, String> {
    // 獲取命令列參數
    let args: Vec<String> = std::env::args().collect();
    
    // 檢查是否有文件路徑參數（通常是第二個參數，索引為 1）
    if let Some(path) = args.get(1) {
        Ok(path.clone())
    } else {
        // 如果沒有提供檔案路徑，返回錯誤
        Err("Markdown file path not provided.".to_string())
    }
}

// 主函數：應用程式的入口點
fn main() {
    // 建立 Tauri 應用程式
    tauri::Builder::default()
        // 設定應用程式初始化
        .setup(|app| {
            // 獲取命令列參數
            let args: Vec<String> = std::env::args().collect();
            // 獲取主視窗
            let window = app.get_window("main").unwrap();
            
            // 如果提供了檔案路徑參數
            if let Some(path) = args.get(1) {
                // 建立一個 Arc 來安全地共享路徑
                let path_arc = Arc::new(path.clone());
                let app_handle_clone = app.handle();
                
                // 設定視窗標題，顯示正在查看的檔案
                window.set_title(format!("Markdown Viewer - {}", path).as_str());
                
                // 發送檔案路徑事件給所有前端監聽器
                app_handle_clone
                    .emit_all("file_path", path.as_str())
                    .unwrap();
                
                // 監聽視窗創建事件，重新發送檔案路徑
                app.listen_global("tauri://window-created", move |_| {
                    let path_clone = path_arc.clone();
                    app_handle_clone
                        .emit_all("file_path", path_clone.as_str())
                        .unwrap();
                });
            }
            
            // 在 Windows 平台上應用 Mica 視窗效果
            #[cfg(target_os = "windows")]
            apply_mica(&window, None)
                .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            
            // 臨時修正方案：處理視窗顯示問題
            // 這是一個暫時的解決方案，希望未來 Tauri 版本能修復
            // https://github.com/tauri-apps/tauri/discussions/4747
            window.minimize().unwrap();    // 先最小化視窗
            window.unminimize().unwrap();  // 取消最小化
            window.maximize().unwrap();    // 最大化視窗
            window.unmaximize().unwrap();  // 取消最大化
            window.show().unwrap();        // 顯示視窗
            window.set_focus().unwrap();   // 設定視窗焦點
            window.set_decorations(true).unwrap(); // 啟用視窗裝飾（標題欄等）
            
            Ok(())
        })
        // 註冊 Tauri 命令處理器
        .invoke_handler(tauri::generate_handler![open_markdown, send_markdown_path])
        // 運行 Tauri 應用程式
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
