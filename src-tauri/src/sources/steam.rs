use steamgriddb_api::{images::Image, query_parameters::Platform, Client, QueryType};
use tauri::State;
use tokio::sync::Mutex;

use crate::models::config::Config;

#[tauri::command]
pub async fn get_image_for_id(
    id: &str,
    state: State<'_, Mutex<Config>>,
) -> Result<Vec<Image>, String> {
    let config = state.lock().await;

    let api = match &config.steamgrid_api_key {
        None => return Ok(Vec::new()),
        Some(a) => a,
    };

    let client = Client::new(api);

    client
        .get_images_for_platform_id(&Platform::Steam, id, &QueryType::Grid(None))
        .await
        .map_err(|e| e.to_string())
}
