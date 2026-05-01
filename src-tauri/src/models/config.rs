use std::fs::File;

use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;

// TODO: save config to disk
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Config {
    pub steamgrid_api_key: Option<String>,
}

impl Config {
    pub fn save_to_disk(&self) -> Result<(), String> {
        let proj_dirs = ProjectDirs::from("dev", "Grenin", "caltrop")?.config_dir();
        let full_path = format!("{config_folder}/caltrop/config.json");
        let test = proj_dirs.join("config.json");

        let mut file = File::create_new(full_path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &self)?;
        Ok(())
    }
}

#[tauri::command]
pub async fn set_steamgrid_key(
    api_key: &str,
    state: State<'_, Mutex<Config>>,
) -> Result<(), String> {
    let mut state = state.lock().await;
    state.steamgrid_api_key = Some(api_key.to_string());
    state.save_to_disk()?;
    Ok(())
}
