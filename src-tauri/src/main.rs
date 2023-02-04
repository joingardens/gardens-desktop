#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
mod services;
mod core;
use services::root_status_getter_service::get_docker_permission_status;
use services::version_getter_service::LinuxVersionGetter;
use tauri::Runtime;

use crate::services::version_getter_service::DockerVersionGetter;
use crate::services::version_getter_service::VersionGetter;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet() -> [String; 3] {
    let docker_status = DockerVersionGetter::version_status().to_string();
    let os_status = LinuxVersionGetter::version_status().to_string();
    let root_status: String = get_docker_permission_status().to_string();
    
    [docker_status, os_status, root_status]

}

#[tauri::command]
async fn begin_installation<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) -> Result<String, String> {
  return Err("no_ok".to_string())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, begin_installation])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
