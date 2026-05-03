pub mod commands;
pub mod http;
pub mod models;

use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;
use tauri_plugin_cli::CliExt;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_autostart::Builder::new().build())
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--silent"])))
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let window = app.get_webview_window("main").expect("no main window");
            if !window.is_visible().unwrap_or(false) {
                let _ = window.show();
            }
            let _ = window.set_focus();
        }))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            let close_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let show_item = MenuItem::with_id(app, "show", "显示主界面", true, None::<&str>)?;
            let tray_menu = Menu::with_items(app, &[&show_item, &close_item])?;

            let mut tray_builder = TrayIconBuilder::new().menu(&tray_menu);
            if let Some(icon) = app.default_window_icon() {
                tray_builder = tray_builder.icon(icon.clone());
            }
            tray_builder.build(app)?;

            match app.cli().matches() {
                Ok(matches) => {
                    if let Some(arg) = matches.args.get("silent") {
                        if arg.value.as_bool().unwrap() {
                            let window = app.get_webview_window("main").unwrap();
                            let _ = window.hide();
                        }
                    }
                    
                }
                Err(_) => {}
            }

            Ok(())
        })
        .on_menu_event(|app, event| {
            if event.id().as_ref() == "quit" {
                app.exit(0);
            }
            if event.id().as_ref() == "show" {
                let window = app.get_webview_window("main").unwrap();
                if !window.is_visible().unwrap_or(false) {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                if let Some(window) = tray.app_handle().get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
            }
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_music_bytes::get_music_bytes,
            commands::request_music_async::request_music_async
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
