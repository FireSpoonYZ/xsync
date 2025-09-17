pub mod conflict;
pub mod engine;
pub mod state;

pub use conflict::ConflictResolver;
pub use engine::SyncEngine;
pub use state::SyncState;

use crate::clipboard::ClipboardEvent;
use serde::{Deserialize, Serialize};

/// Synchronization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    pub auto_sync: bool,
    pub sync_text: bool,
    pub sync_images: bool,
    pub sync_files: bool,
    pub max_file_size: u64,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            auto_sync: true,
            sync_text: true,
            sync_images: true,
            sync_files: true,
            max_file_size: 1_073_741_824, // 1GB
        }
    }
}

/// Sync operation result
#[derive(Debug)]
pub enum SyncResult {
    Success,
    Conflict(ClipboardEvent),
    Error(String),
}