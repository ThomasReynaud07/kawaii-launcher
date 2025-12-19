use crate::commands::downloader;
use crate::commands::version::{Version, get_version};
use std::fs;
use std::path::{absolute, Path};
use std::process::Command;

fn create_folders() {
    let folders = [
        "minecraft",
        "minecraft/assets",
        "minecraft/libraries",
        "minecraft/versions",
        "minecraft/bin",
    ];
    for folder in folders {
        if !Path::new(folder).exists() {
            fs::create_dir(folder).expect("Could not create folder");
        }
    }
}

fn get_libraries(version: Version) -> Vec<String> {
    let mut libraries = Vec::new();
    let libraries_path = Path::new("minecraft/libraries");
    for lib in version.libraries {
        libraries.push(libraries_path.join(lib.downloads.artifact.unwrap().path).to_string_lossy().to_string());
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
    let natives_folder = "minecraft/bin";
    let version = get_version(version).await;
    let libraries = get_libraries(version.clone());
    let _separator = ":";
    #[cfg(target_os = "windows")]
    let _separator = ";";

    let classpath = format!(
        "{}/{}/{}.jar{}{}",
        version_folder,
        &version.id,
        &version.id,
        _separator,
        libraries.join(_separator)
    );

    let _command = "java";
    #[cfg(target_os = "windows")]
    let _command = "javaw";

    Command::new(_command)
    .args(&[
        #[cfg(target_os = "macos")]
        "-XstartOnFirstThread",
        #[cfg(target_os = "windows")]
        "-XX:HeapDumpPath=MojangTricksIntelDriversForPerformance_javaw.exe_minecraft.exe.heapdump", 
        #[cfg(target_arch = "x86")]
        "-Xss1M",
        &format!("-Djava.library.path={}", natives_folder),
        &format!("-Djna.tmpdir={}", natives_folder),
        &format!("-Dorg.lwjgl.system.SharedLibraryExtractPath={}", natives_folder),
        &format!("-Dio.netty.native.workdir={}", natives_folder),
        &format!("-Dminecraft.launcher.brand={}", "Kawaii"),
        &format!("-Dminecraft.launcher.version={}", 100),
        "-cp",
        &classpath,
        &version.main_class,
        "--username",
        &username, 
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
