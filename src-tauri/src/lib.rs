mod metadata;
mod models;
mod sources;
mod tauri_bridge;

use models::config::Config;
use tauri::Manager;
use tokio::sync::Mutex;

use crate::models::app_state::AppState;
use crate::sources::steam::SteamSource;

use crate::tauri_bridge::{
    config_bridge::{get_steamgrid_key, set_steamgrid_key},
    meta_bridge::get_image_for_id,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                //.level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            let config = Config::load_from_disk().unwrap_or_default();
            app.manage(Mutex::new(AppState {
                config: config,
                sources: vec![Box::new(SteamSource::new())],
            }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_image_for_id,
            set_steamgrid_key,
            get_steamgrid_key,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
