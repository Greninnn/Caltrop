use serde::{Deserialize, Serialize};

use crate::sources::source_trait::Source;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub source: Source,
    pub install_path: Option<String>,
}

impl Game {
    fn get_unique_id(&self) -> String {
        format!("{}:{}", self.source, self.id)
    }
}