// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod command_registry;
mod macros;
mod flv;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init()) // 初始化 Dialog 插件
        .invoke_handler(command_registry::collect_commands())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
