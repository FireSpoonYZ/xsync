// Clipboard reading functionality

use crate::clipboard::{ClipboardContent, ClipboardEvent, ImageFormat};
use anyhow::Result;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(target_os = "linux")]
mod linux {
    use super::*;
    use x11_clipboard::Clipboard;
    
    pub struct ClipboardReader {
        clipboard: Clipboard,
        device_id: String,
    }
    
    impl ClipboardReader {
        pub fn new() -> Result<Self> {
            let clipboard = Clipboard::new()?;
            let device_id = uuid::Uuid::new_v4().to_string();
            Ok(Self { clipboard, device_id })
        }
        
        pub async fn read_clipboard(&mut self) -> Result<Option<ClipboardEvent>> {
            // Try reading text first
            if let Ok(text_data) = self.clipboard.load(
                self.clipboard.getter.atoms.clipboard,
                self.clipboard.getter.atoms.utf8_string,
                self.clipboard.getter.atoms.property,
                std::time::Duration::from_millis(1000),
            ) {
                let text = String::from_utf8_lossy(&text_data).to_string();
                if !text.is_empty() {
                    return Ok(Some(self.create_event(ClipboardContent::Text(text))));
                }
            }
            
            // Try reading image data (PNG format)
            let png_atom = self.clipboard.getter.get_atom("image/png")?;
            if let Ok(image_data) = self.clipboard.load(
                self.clipboard.getter.atoms.clipboard,
                png_atom,
                self.clipboard.getter.atoms.property,
                std::time::Duration::from_millis(1000),
            ) {
                if !image_data.is_empty() {
                    return Ok(Some(self.create_event(ClipboardContent::Image {
                        format: ImageFormat::Png,
                        data: image_data,
                    })));
                }
            }
            
            Ok(None)
        }
        
        fn create_event(&self, content: ClipboardContent) -> ClipboardEvent {
            let content_bytes = match &content {
                ClipboardContent::Text(text) => text.as_bytes(),
                ClipboardContent::Image { data, .. } => data.as_slice(),
                ClipboardContent::Files { .. } => b"", // TODO: Handle files
            };
            
            let content_hash = {
                use sha2::{Digest, Sha256};
                let mut hasher = Sha256::new();
                hasher.update(content_bytes);
                format!("{:x}", hasher.finalize())
            };
            
            ClipboardEvent {
                content,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
                device_id: self.device_id.clone(),
                content_hash,
            }
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod fallback {
    use super::*;
    
    pub struct ClipboardReader {
        device_id: String,
    }
    
    impl ClipboardReader {
        pub fn new() -> Result<Self> {
            Ok(Self {
                device_id: uuid::Uuid::new_v4().to_string(),
            })
        }
        
        pub async fn read_clipboard(&mut self) -> Result<Option<ClipboardEvent>> {
            log::warn!("Clipboard reading not implemented for this platform");
            Ok(None)
        }
    }
}

#[cfg(target_os = "linux")]
pub use linux::ClipboardReader;

#[cfg(not(target_os = "linux"))]
pub use fallback::ClipboardReader;