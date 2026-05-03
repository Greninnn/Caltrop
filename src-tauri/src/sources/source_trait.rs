use std::{fmt::Formatter};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::models::{error::SourceError, game::Game};

#[async_trait]
pub trait GameSource {
    fn name(&self) -> Source;

    async fn get_installed_games(&self) -> Result<Vec<Game>, SourceError>;
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Source {
    Steam,
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}")
    }
}