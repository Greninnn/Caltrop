use tauri::State;
use tokio::sync::Mutex;

use crate::models::{app_state::AppState};

#[tauri::command]
pub async fn set_steamgrid_key(
    api_key: &str,
    state: State<'_, Mutex<AppState>>,
) -> Result<(), String> {
    log::debug!("got request to update api key to {api_key}");
    let mut state = state.lock().await;
    state.config.steamgrid_api_key = Some(api_key.to_string());

    state.config.save_to_disk().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_steamgrid_key(state: State<'_, Mutex<AppState>>) -> Result<Option<String>, String> {
    let state = state.lock().await;
    Ok(state.config.steamgrid_api_key.clone())
}
