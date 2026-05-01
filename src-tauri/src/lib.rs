mod models;
mod sources;

use tauri::Manager;
use tokio::sync::Mutex;
use models::config::Config;

use crate::models::config::set_steamgrid_key;
use crate::sources::steam::get_image_for_id;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(Config::default()));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_image_for_id, set_steamgrid_key])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
