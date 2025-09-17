// Conflict resolution logic

use crate::clipboard::ClipboardEvent;
use crate::sync::SyncResult;

pub struct ConflictResolver;

impl ConflictResolver {
    pub fn new() -> Self {
        Self
    }
    
    pub fn resolve(&self, local_event: &ClipboardEvent, remote_event: &ClipboardEvent) -> SyncResult {
        // Use timestamp-based conflict resolution
        if remote_event.timestamp > local_event.timestamp {
            SyncResult::Conflict(remote_event.clone())
        } else {
            SyncResult::Success
        }
    }
}