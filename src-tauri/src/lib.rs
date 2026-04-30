mod models;

use std::iter::Once;
use std::string;
use tauri::State;
use tauri::Manager;
use tokio::sync::Mutex;
use models::config::Config;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn set_steamgrid_key(api_key: &str, state: State<'_, Mutex<Config>>) -> Result<(), String> {
    let mut state = state.lock().await;
    state.steamgrid_api_key = Some(api_key.to_string());
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(Config::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
