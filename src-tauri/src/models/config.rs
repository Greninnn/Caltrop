use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

use directories_next::ProjectDirs;
use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;

use crate::models::error::ConfigError;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Config {
    pub steamgrid_api_key: Option<String>,
}

impl Config {
    pub fn save_to_disk(&self) -> Result<(), ConfigError> {
        let binding =
            ProjectDirs::from("dev", "Grenin", "caltrop").ok_or(ConfigError::NoHomeDir)?;
        let proj_dirs = binding.config_dir();

        let full_path = proj_dirs.join("config.json");

        std::fs::create_dir_all(proj_dirs)?;

        let file = File::create(full_path)?;
        let writer = BufWriter::new(file);

        serde_json::to_writer_pretty(writer, &self)?;

        log::debug!("wrote config: {}", serde_json::to_string_pretty(self)?);

        Ok(())
    }

    pub fn load_from_disk() -> Result<Self, ConfigError> {
        let proj_dirs =
            ProjectDirs::from("dev", "Grenin", "caltrop").ok_or(ConfigError::NoHomeDir)?;
        let config_dir = proj_dirs.config_dir();

        let full_path = config_dir.join("config.json");

        if !full_path.exists() {
            return Ok(Self::default());
        }

        let file = File::open(full_path)?;
        let reader = BufReader::new(file);

        let config = serde_json::from_reader(reader)?;

        log::debug!("loaded config: {}", serde_json::to_string_pretty(&config)?);

        Ok(config)
    }
}

#[tauri::command]
pub async fn set_steamgrid_key(
    api_key: &str,
    state: State<'_, Mutex<Config>>,
) -> Result<(), String> {
    log::debug!("got request to update api key to {api_key}");
    let mut state = state.lock().await;
    state.steamgrid_api_key = Some(api_key.to_string());

    state.save_to_disk().map_err(|e| e.to_string())
}
