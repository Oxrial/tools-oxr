use flvparse::{FlvFile, FlvTag, FlvTagData};
use std::fs;
use std::io::Write;
use std::process::Command;
use tauri::command;

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
pub fn generate_filelist_and_merge(files: Vec<String>, folder_path: String, file_name: String) {
    for file in &files {
        let bytes = std::fs::read(&file).expect("读取文件失败");
        let (_, header) = flvparse::FlvFileBody::parse(&bytes).unwrap();
        // 使用 flvparse 解析

        // 提取信息
        let info = format!(
            "FLV文件基本信息：{}\n\
            - first_previous_tag_size: {}",
            file, header.first_previous_tag_size
        );
        println!("{:?}", header.tags);
        println!("info {}", info);
    }
    // let mut filelist = fs::File::create(format!("{}/filelist.txt", folder_path)).unwrap();
    // for file in &files {
    // writeln!(filelist, "file '{}'", newfile).unwrap();
    // }

    // let output_path = format!("{}/../{}.mp4", folder_path, file_name);
    // let status = Command::new("ffmpeg")
    //     .args([
    //         "-f",
    //         "concat",
    //         "-safe",
    //         "0",
    //         "-i",
    //         &format!("{}/filelist.txt", folder_path),
    //         "-c",
    //         "copy",
    //         &output_path,
    //     ])
    //     .status()
    //     .expect("FFmpeg 执行失败");

    // if !status.success() {
    //     panic!("FFmpeg 合并失败");
    // }
}
fn fix(file: String) -> std::string::String {
    let newfile = format!("{}_fix.flv", file);
    let status = Command::new("ffmpeg")
    .args([
        "-i",
        &file,
        "-vf",
        "scale=1920:1080:force_original_aspect_ratio=decrease,pad=1920:1080:(ow-iw)/2:(oh-ih)/2,fps=60",
        "-c:v",
        "libx264",
        "-preset",
        "medium",
        "-profile:v",
        "high","-level","4.1","-c:a","aac","-b:a","320k","-flvflags","+add_keyframe_index","-f","flv",
        &newfile,
    ])
    .status()
    .expect("FFmpeg 执行失败");

    if !status.success() {
        panic!("FFmpeg 合并失败");
    }
    return newfile;
}
#[command]
pub fn conver_ext(folder_path: String, input_name: String, ext: String) {
    //ffmpeg -i '001-Ferrari__Instrumenta.mp4' -vn -ar 44100 -ac 2 -ab 320k 'ferrari_banzou.wav'
    let input_path = format!("'{}/{}'", folder_path, input_name);
    let output_path = format!("'{}/{}.{}'", folder_path, input_name, ext);
    let status = Command::new("ffmpeg")
        .args([
            "-i",
            &input_path,
            "-vn",
            "-ar",
            "44100",
            "-ac",
            "2",
            "-ab",
            "320k",
            &output_path,
        ])
        .status()
        .expect("FFmpeg 执行失败");

    if !status.success() {
        panic!("FFmpeg 转换失败");
    }
}
