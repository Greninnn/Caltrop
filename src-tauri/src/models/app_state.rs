use crate::{models::config::Config, sources::source_trait::GameSource};

pub struct AppState {
    pub config: Config,
    pub sources: Vec<Box<dyn GameSource + Send + Sync>>,
}