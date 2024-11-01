use tauri::{
    image::Image, tray::{MouseButton, TrayIconBuilder, TrayIconEvent}, Emitter, Manager, Runtime
};
// use std::thread::{sleep};
// use std::time::Duration;
// use egui::include_image;

pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
    // const APP_ICON: Image<'_> = include_image!("../tray/icoc.png");

    let _ = TrayIconBuilder::with_id("tray")
        .tooltip("tauri")
        .icon(app.default_window_icon().unwrap().clone())
        .on_tray_icon_event(|tray, event| match event {
            TrayIconEvent::Click {
                id: _,
                position,
                rect: _,
                button,
                button_state: _,
            } => match button {
                MouseButton::Left {} => {
                    // ...
                    print!("Tray icon was clicked at position: ({}, {})", position.x, position.y);
                    let app = tray.app_handle();
                    app.get_webview_window("main").unwrap().show().unwrap();
                    // if let Some(window) = app.get_webview_window("main") {
                    //   let _ = window.show();
                    //   let _ = window.set_focus();
                    // }
                }
                MouseButton::Right {} => {
                    tray.app_handle().emit("tray_contextmenu", position).unwrap();
                }
                _ => {}
            },
            TrayIconEvent::Enter {
                id: _,
                position,
                rect: _,
            } => {
                tray.app_handle().emit("tray_mouseenter", position).unwrap();
            }
            TrayIconEvent::Leave {
                id: _,
                position,
                rect: _,
            } => {
                // sleep(Duration::from_millis(500));
                tray.app_handle().emit("tray_mouseleave", position).unwrap();
            }
            _ => {}
        })
        .build(app);
    Ok(())
}


// use tauri::{
//     menu::{Menu, MenuItem},
//     tray::TrayIconBuilder,
//     Manager, Runtime,
// };
// use tauri::tray::TrayIconBuilder;

// pub fn create_tray<R: Runtime>(app: &tauri::AppHandle<R>) -> tauri::Result<()> {
//     // 创建关闭菜单项
//     let quit_i = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
//     // 创建关于菜单项
//     let about_i = MenuItem::with_id(app, "about", "关于", true, None::<&str>)?;
//     // 创建菜单
//     let menu = Menu::with_items(app, &[&quit_i, &about_i])?;

//     let _tray = TrayIconBuilder::new()
//         .icon(app.default_window_icon().unwrap().clone())
//         .menu(&menu)
//         .menu_on_left_click(false)
//         .build(app)?;
//     _tray.set_icon("tray/msg.png");

//     TrayIconBuilder::new()
//         .on_menu_event(|app, event| match event.id.as_ref() {
//             "quit" => {
//                 println!("quit menu item was clicked");
//                 app.exit(0);
//             }
//             "about" => {
//                 println!("about menu item was clicked");
//                 // 打开关于窗口
//                 let window = app.get_webview_window("main").unwrap();
//                 window.show().unwrap();
//                 window.set_focus().unwrap();
//             }
//             _ => {
//                 println!("menu item {:?} not handled", event.id);
//             }
//         })
//         .build(app)?;

//     // 获取主窗口的 webview
//     let window = app.get_webview_window("main").unwrap();

//     // 在关闭窗口时清理托盘图标
//     window.on_window_event(move |event| {
//         if let tauri::WindowEvent::CloseRequested { .. } = event {
//             // 直接通过托盘的事件处理销毁应用
//             std::process::exit(0);
//         }
//     });

//     Ok(())
// }
