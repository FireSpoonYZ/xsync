pub mod discovery;
pub mod protocol;
pub mod tcp_client;
pub mod tcp_server;

pub use discovery::DiscoveryService;
pub use protocol::{Message, MessageType};
pub use tcp_client::TcpClient;
pub use tcp_server::TcpServer;

use serde::{Deserialize, Serialize};

/// Peer information for network discovery
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerInfo {
    pub device_id: String,
    pub device_name: String,
    pub tcp_port: u16,
    pub capabilities: Vec<String>,
    pub last_seen: u64,
}

/// Network configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub discovery_port: u16,
    pub tcp_port: u16,
    pub multicast_addr: String,
    pub heartbeat_interval: u64,
    pub peer_timeout: u64,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            discovery_port: 52525,
            tcp_port: 52526,
            multicast_addr: "239.255.255.250".to_string(),
            heartbeat_interval: 5,
            peer_timeout: 15,
        }
    }
}