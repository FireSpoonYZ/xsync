use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::fs;

use crate::network::NetworkConfig;
use crate::storage::StorageConfig;
use crate::sync::SyncConfig;

/// Main application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub network: NetworkConfig,
    pub sync: SyncConfig,
    pub storage: StorageConfig,
    pub device_name: String,
}

impl Default for Config {
    fn default() -> Self {
        let hostname = hostname::get()
            .unwrap_or_else(|_| std::ffi::OsString::from("unknown"))
            .to_string_lossy()
            .to_string();

        Self {
            network: NetworkConfig::default(),
            sync: SyncConfig::default(),
            storage: StorageConfig::default(),
            device_name: hostname,
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub async fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;
        
        if config_path.exists() {
            let contents = fs::read_to_string(&config_path).await?;
            let config: Config = toml::from_str(&contents)?;
            Ok(config)
        } else {
            let config = Config::default();
            config.save().await?;
            Ok(config)
        }
    }
    
    /// Save configuration to file
    pub async fn save(&self) -> Result<()> {
        let config_path = Self::config_file_path()?;
        
        // Ensure directory exists
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).await?;
        }
        
        let contents = toml::to_string_pretty(self)?;
        fs::write(&config_path, contents).await?;
        
        Ok(())
    }
    
    /// Get the path to the configuration file
    fn config_file_path() -> Result<PathBuf> {
        let config_dir = directories::ProjectDirs::from("", "", "xsync")
            .ok_or_else(|| anyhow::anyhow!("Failed to get config directory"))?
            .config_dir()
            .to_path_buf();
        
        Ok(config_dir.join("config.toml"))
    }
}