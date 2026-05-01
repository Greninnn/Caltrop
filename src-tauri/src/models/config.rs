use serde::{Deserialize, Serialize};
use tauri::State;
use tokio::sync::Mutex;

// TODO: save config to disk
#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Config {
    pub steamgrid_api_key: Option<String>,
}

#[tauri::command]
pub async fn set_steamgrid_key(api_key: &str, state: State<'_, Mutex<Config>>) -> Result<(), String> {
    let mut state = state.lock().await;
    state.steamgrid_api_key = Some(api_key.to_string());
    Ok(())
}
