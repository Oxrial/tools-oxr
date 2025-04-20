use std::fs;
use std::io::Write;
use std::process::Command;
use tauri::command;
use crate::macros::tauri_command;

#[command]
pub fn scan_flv_files(path: String) -> Vec<String> {
    match fs::read_dir(&path) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "flv"))
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .collect(),
        Err(e) => {
            eprint!("读取目录时发生错误: {}", e);
            Vec::new()
        }
    }
}

#[command]
pub fn generate_filelist_and_merge(files: Vec<String>, folder_path: String) {
    let mut filelist = fs::File::create(format!("{}/filelist.txt", folder_path)).unwrap();
    for file in &files {
        writeln!(filelist, "file '{}'", file).unwrap();
    }

    let output_path = format!("{}/output.mp4", folder_path);
    let status = Command::new("ffmpeg")
        .args([
            "-f",
            "concat",
            "-safe",
            "0",
            "-i",
            &format!("{}/filelist.txt", folder_path),
            "-c",
            "copy",
            &output_path,
        ])
        .status()
        .expect("FFmpeg 执行失败");

    if !status.success() {
        panic!("FFmpeg 合并失败");
    }
}

// pub fn run() {
//     tauri::Builder::default()
//         .plugin(tauri_plugin_dialog::init()) // 初始化 Dialog 插件
//         .invoke_handler(tauri::generate_handler![
//             scan_flv_files,
//             generate_filelist_and_merge
//         ])
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }
tauri_command!(scan_flv_files);
tauri_command!(generate_filelist_and_merge);