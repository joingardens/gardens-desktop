#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod services;
mod core;

use services::version_getter_service::LinuxVersionGetter;

use crate::services::version_getter_service::DockerVersionGetter;
use crate::services::version_getter_service::VersionGetter;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> [String; 2] {
    let docker_status = DockerVersionGetter::version_status().to_string();
    let os_status = LinuxVersionGetter::version_status().to_string();
    
    [docker_status, os_status]

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
