// Local state management

use crate::clipboard::ClipboardEvent;
use std::collections::HashMap;

pub struct SyncState {
    current_clipboard: Option<ClipboardEvent>,
    peer_states: HashMap<String, ClipboardEvent>,
}

impl SyncState {
    pub fn new() -> Self {
        Self {
            current_clipboard: None,
            peer_states: HashMap::new(),
        }
    }
    
    pub fn update_local_clipboard(&mut self, event: ClipboardEvent) {
        self.current_clipboard = Some(event);
    }
    
    pub fn update_peer_state(&mut self, peer_id: String, event: ClipboardEvent) {
        self.peer_states.insert(peer_id, event);
    }
    
    pub fn get_current_clipboard(&self) -> &Option<ClipboardEvent> {
        &self.current_clipboard
    }
}