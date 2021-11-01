use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Could not find configuration {0:?}")]
    FileNotFound(PathBuf),

    #[error("Could not find configurations in folder {0:?}")]
    FolderNotFound(PathBuf),

    #[error("Could not read configuration file")]
    FileReadFailed(#[from] std::io::Error),

    #[error("Failed to parse file as a toml file")]
    FileTomlParseFailed(#[from] toml::de::Error),
}
