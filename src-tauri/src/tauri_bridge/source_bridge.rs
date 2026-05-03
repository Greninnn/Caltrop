use tauri::State;
use tokio::{sync::Mutex, task::JoinSet};

use crate::models::{app_state::AppState, error::SourceError, game::Game};

#[tauri::command]
pub async fn get_all_installed_games(
    state: State<'_, Mutex<AppState>>,
) -> Result<Vec<Game>, SourceError> {
    let state = state.lock().await;

    // Also need to store imported games somewhere. Maybe the "data" folder from dirs?
    todo!("aggregate all games into one list");
}
