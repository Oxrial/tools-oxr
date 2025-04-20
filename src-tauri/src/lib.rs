use tauri::command;
use tauri_plugin_dialog::DialogExt; // 引入 Dialog 插件


#[command]
fn select_folder(app_handle: tauri::AppHandle) -> String {
    let dialog = app_handle.dialog();
    dialog
        .file()
        .blocking_pick_folder()
        .map(|path| path.to_string()) // 使用 FilePath::to_string() 方法
        .unwrap_or_else(|| "".into())
}

tauri_command!(select_folder);

// mod flv;
// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_dialog::init()) // 初始化 Dialog 插件
//         .invoke_handler(tauri::generate_handler![
//             flv::scan_flv_files,
//             flv::generate_filelist_and_merge,
//             // select_folder,
//         ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
