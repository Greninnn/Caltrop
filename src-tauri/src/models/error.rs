use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Could not find valid home directory")]
    NoHomeDir,

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}
