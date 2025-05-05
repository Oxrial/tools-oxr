// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::command;
use tauri_plugin_dialog::DialogExt; // 引入 Dialog 插件
mod flv;

#[command]
fn select_folder(app_handle: tauri::AppHandle) -> String {
    let dialog = app_handle.dialog();
    dialog
        .file()
        .blocking_pick_folder()
        .map(|path| path.to_string()) // 使用 FilePath::to_string() 方法
        .unwrap_or_else(|| "".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            select_folder,
            flv::scan_flv_files,
            flv::generate_filelist_and_merge,
            flv::conver_ext
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
