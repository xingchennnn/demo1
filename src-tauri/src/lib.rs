// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
// use serde::Serialize;
// use tauri::Emitter;
// use tauri::Manager;

mod commands;
mod tray;

#[tauri::command]
fn greet(name: &str) -> String {


    println!("rust接收到的参数 ,{}", name);
    format!("rust 返回参数 {}", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                // let docs_window = tauri::WindowBuilder::new(
                //     app,
                //     "external", /* the unique window label */
                //     tauri::WindowUrl::External("https://tauri.app/".parse().unwrap())
                //   ).build()?;

                let handle = app.handle();
                tray::create_tray(&handle.clone())?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet, commands::close ,commands::open_devtools])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// #[derive(Clone, Serialize)]
// #[serde(rename_all = "camelCase")]
// struct DownloadStarted<'a> {
//     url: &'a str,
//     download_id: usize,
//     content_length: usize,
// }
