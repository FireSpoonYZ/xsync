// Clipboard writing functionality

use crate::clipboard::{ClipboardContent, ClipboardEvent, ImageFormat};
use anyhow::Result;

#[cfg(target_os = "linux")]
mod linux {
    use super::*;
    use x11_clipboard::Clipboard;
    
    pub struct ClipboardWriter {
        clipboard: Clipboard,
    }
    
    impl ClipboardWriter {
        pub fn new() -> Result<Self> {
            let clipboard = Clipboard::new()?;
            Ok(Self { clipboard })
        }
        
        pub async fn write_clipboard(&mut self, event: &ClipboardEvent) -> Result<()> {
            match &event.content {
                ClipboardContent::Text(text) => {
                    self.clipboard.store(
                        self.clipboard.setter.atoms.clipboard,
                        self.clipboard.setter.atoms.utf8_string,
                        text.as_bytes(),
                    )?;
                }
                ClipboardContent::Image { format, data } => {
                    let target_atom = match format {
                        ImageFormat::Png => self.clipboard.setter.get_atom("image/png")?,
                        ImageFormat::Jpeg => self.clipboard.setter.get_atom("image/jpeg")?,
                        ImageFormat::Bmp => {
                            // BMP might not be directly supported, try as binary
                            self.clipboard.setter.get_atom("image/bmp")
                                .unwrap_or_else(|_| self.clipboard.setter.get_atom("image/png").unwrap())
                        }
                    };
                    
                    self.clipboard.store(
                        self.clipboard.setter.atoms.clipboard,
                        target_atom,
                        data.clone(),
                    )?;
                }
                ClipboardContent::Files { paths: _ } => {
                    // TODO: Implement file clipboard handling
                    log::warn!("File clipboard content not yet implemented");
                }
            }
            
            Ok(())
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod fallback {
    use super::*;
    
    pub struct ClipboardWriter;
    
    impl ClipboardWriter {
        pub fn new() -> Result<Self> {
            Ok(Self)
        }
        
        pub async fn write_clipboard(&mut self, _event: &ClipboardEvent) -> Result<()> {
            log::warn!("Clipboard writing not implemented for this platform");
            Ok(())
        }
    }
}

#[cfg(target_os = "linux")]
pub use linux::ClipboardWriter;

#[cfg(not(target_os = "linux"))]
pub use fallback::ClipboardWriter;