use steamgriddb_api::{Client, QueryType, images::Image, query_parameters::Platform};

use crate::models::config::Config;

pub async fn get_images_for_id(
    id: &str,
    config: &Config,
) -> Result<Vec<Image>, String> {

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