use crate::commands::downloader;
use crate::commands::version::get_version;
use std::fs;
use std::path::{absolute, Path};
use std::process::Command;

fn create_folders() {
    let folders = [
        "minecraft",
        "minecraft/assets",
        "minecraft/libraries",
        "minecraft/versions",
        "minecraft/natives",
    ];
    for folder in folders {
        if !Path::new(folder).exists() {
            fs::create_dir(folder).expect("Could not create folder");
        }
    }
}

fn get_libraries(dir: &Path) -> Vec<String> {
    let mut libraries = Vec::new();
    let dir = fs::read_dir(dir).expect("Failed to read dir");
    for file in dir {
        let path = file.unwrap().path();
        if path.is_dir() {
            libraries.extend(get_libraries(&path));
        } else {
            let absolute = absolute(&path).unwrap().to_string_lossy().to_string();
            libraries.push(absolute);
        }
    }
    libraries
}

#[tauri::command]
pub async fn launch_game(username: String, version: String) {
    create_folders();
    downloader::start_download(get_version(version.clone()).await).await;
    println!(
        "Start launching the game as {} in {}!",
        username,
        version.clone()
    );
    let base_folder = "minecraft/";
    let library_folder = "minecraft/libraries";
    let assets_folder = "minecraft/assets";
    let version_folder = "minecraft/versions";
    let natives_folder = "minecraft/natives";
    let version = get_version(version).await;
    let libraries = get_libraries(Path::new(library_folder));
    let classpath = format!(
        "{}/{}/{}.jar{}{}",
        version_folder,
        &version.id,
        &version.id,
        ";",
        libraries.join(";")
    );

    Command::new("java")
    .args(&[
        "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump", 
        "-Xss1M",
        &format!("-Djava.library.path={}", natives_folder),
        &format!("-Djna.tmpdir={}", natives_folder),
        &format!("-Dorg.lwjgl.system.SharedLibraryExtractPath={}", natives_folder),
        &format!("-Dio.netty.native.workdir={}/natives", base_folder),
        &format!("-Dminecraft.launcher.brand={}", "Kawaii"),
        &format!("-Dminecraft.launcher.version={}", 100),
        "-cp",
        &classpath,
        &version.main_class,
        "--username",
        "player", 
        "--version",
        &version.id,
        "--gameDir",
        base_folder,
        "--assetsDir",
        assets_folder,
        "--assetIndex",
        &version.asset_index.id,
        "--uuid",
        "00000000-0000-0000-0000-000000000000",
        "--accessToken",
        "0",
        "--versionType",
        &version.r#type,
    ])
    .spawn()
    .expect("Failed to run the game");
}
