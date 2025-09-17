pub mod monitor;
pub mod reader;
pub mod writer;

pub use monitor::ClipboardMonitor;
pub use reader::ClipboardReader;
pub use writer::ClipboardWriter;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents different types of clipboard content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClipboardContent {
    Text(String),
    Image {
        format: ImageFormat,
        data: Vec<u8>,
    },
    Files {
        paths: Vec<PathBuf>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageFormat {
    Png,
    Jpeg,
    Bmp,
}

/// Clipboard change event with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardEvent {
    pub content: ClipboardContent,
    pub timestamp: u64,
    pub device_id: String,
    pub content_hash: String,
}