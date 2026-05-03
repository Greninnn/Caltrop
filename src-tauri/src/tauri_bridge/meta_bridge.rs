use tauri::State;
use tokio::sync::Mutex;

use crate::{metadata, models::app_state::AppState};

#[tauri::command]
pub async fn get_image_for_id(id: &str, state: State<'_, Mutex<AppState>>) -> Result<String, String> {
    let config = &state.lock().await.config;
    let images = metadata::images::get_images_for_id(id, config).await?;
    Ok(images[0].url.clone())
}