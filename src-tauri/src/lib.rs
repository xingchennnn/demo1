// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Serialize;
use tauri::Emitter;

mod commands;
mod tray;

#[tauri::command]
fn greet(name: &str) -> String {
    // 打开一个窗口
    // api::window::create("Hello", None);
    // 发送一个通知
    // let mut em = Emitter::new("notification");
    // em.send("Hello, Rust! You've been greeted from Rust!", "info");

    println!("rust接收到的参数 ,{}", name);
    format!("rust 返回参数 {}", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                tray::create_tray(handle)?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, commands::close])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct DownloadStarted<'a> {
    url: &'a str,
    download_id: usize,
    content_length: usize,
}