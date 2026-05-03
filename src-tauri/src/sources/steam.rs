use async_trait::async_trait;

use crate::{
    models::{error::SourceError, game::Game},
    sources::source_trait::{GameSource, Source},
};

pub struct SteamSource;

impl SteamSource {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl GameSource for SteamSource {
    fn name(&self) -> Source {
        Source::Steam
    }

    async fn get_installed_games(&self) -> Result<Vec<Game>, SourceError> {
        todo!("")
    }
}
