// Clipboard monitoring functionality
// Platform-specific implementation for Linux

use crate::clipboard::ClipboardEvent;
use anyhow::Result;
use tokio::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(target_os = "linux")]
mod linux {
    use super::*;
    use x11_clipboard::Clipboard;
    use std::thread;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    pub struct ClipboardMonitor {
        handle: tokio::task::JoinHandle<()>,
    }
    
    impl ClipboardMonitor {
        pub async fn new(sender: mpsc::UnboundedSender<ClipboardEvent>) -> Result<Self> {
            let sender = Arc::new(Mutex::new(sender));
            
            let handle = tokio::spawn(async move {
                // Run clipboard monitoring in a separate thread since x11-clipboard is blocking
                let sender_clone = sender.clone();
                let join_handle = tokio::task::spawn_blocking(move || {
                    let clipboard = Clipboard::new().expect("Failed to create clipboard");
                    let mut last_content = String::new();
                    let device_id = uuid::Uuid::new_v4().to_string();
                    
                    loop {
                        // Try to read clipboard content
                        if let Ok(content) = clipboard.load(
                            clipboard.getter.atoms.clipboard,
                            clipboard.getter.atoms.utf8_string,
                            clipboard.getter.atoms.property,
                            std::time::Duration::from_millis(1000),
                        ) {
                            let content_str = String::from_utf8_lossy(&content).to_string();
                            
                            // Check if content has changed
                            if content_str != last_content && !content_str.is_empty() {
                                last_content = content_str.clone();
                                
                                let event = ClipboardEvent {
                                    content: crate::clipboard::ClipboardContent::Text(content_str.clone()),
                                    timestamp: SystemTime::now()
                                        .duration_since(UNIX_EPOCH)
                                        .unwrap()
                                        .as_secs(),
                                    device_id: device_id.clone(),
                                    content_hash: {
                                        use sha2::{Digest, Sha256};
                                        let mut hasher = Sha256::new();
                                        hasher.update(content_str.as_bytes());
                                        format!("{:x}", hasher.finalize())
                                    },
                                };
                                
                                // Send event (need to handle async in blocking context)
                                if let Ok(sender_guard) = sender_clone.try_lock() {
                                    let _ = sender_guard.send(event);
                                }
                            }
                        }
                        
                        // Sleep to avoid high CPU usage
                        thread::sleep(std::time::Duration::from_millis(500));
                    }
                });
                
                let _ = join_handle.await;
            });
            
            Ok(Self { handle })
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod fallback {
    use super::*;
    
    pub struct ClipboardMonitor {
        _handle: tokio::task::JoinHandle<()>,
    }
    
    impl ClipboardMonitor {
        pub async fn new(_sender: mpsc::UnboundedSender<ClipboardEvent>) -> Result<Self> {
            log::warn!("Clipboard monitoring not implemented for this platform");
            let handle = tokio::spawn(async {
                // Placeholder for unsupported platforms
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
                }
            });
            
            Ok(Self { _handle: handle })
        }
    }
}

#[cfg(target_os = "linux")]
pub use linux::ClipboardMonitor;

#[cfg(not(target_os = "linux"))]
pub use fallback::ClipboardMonitor;