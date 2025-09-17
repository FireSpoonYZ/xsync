// Protocol message definitions

use serde::{Deserialize, Serialize};
use crate::clipboard::ClipboardEvent;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    SyncRequest,
    SyncResponse(ClipboardEvent),
    ClipboardUpdate(ClipboardEvent),
    FileRequest(String), // hash
    FileData { hash: String, chunk: Vec<u8>, offset: u64 },
    Ack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub message_type: MessageType,
    pub timestamp: u64,
    pub device_id: String,
}

impl Message {
    pub fn new(message_type: MessageType, device_id: String) -> Self {
        Self {
            message_type,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            device_id,
        }
    }
}