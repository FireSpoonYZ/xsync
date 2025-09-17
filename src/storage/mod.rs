pub mod cache;
pub mod config;

pub use cache::FileCache;
pub use config::Config;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Storage configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub cache_dir: PathBuf,
    pub receive_dir: PathBuf,
}

impl Default for StorageConfig {
    fn default() -> Self {
        let home = directories::UserDirs::new()
            .map(|dirs| dirs.home_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from("./"));

        Self {
            cache_dir: home.join(".xsync").join("cache"),
            receive_dir: home.join("Downloads").join("xsync"),
        }
    }
}