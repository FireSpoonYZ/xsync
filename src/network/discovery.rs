// UDP multicast peer discovery

use crate::network::{NetworkConfig, PeerInfo};
use anyhow::Result;
use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::time::{interval, Duration};

/// Discovery packet format
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct DiscoveryPacket {
    version: String,
    device_id: String,
    device_name: String,
    tcp_port: u16,
    capabilities: Vec<String>,
}

pub struct DiscoveryService {
    config: NetworkConfig,
    peers: HashMap<String, PeerInfo>,
    device_id: String,
    device_name: String,
}

impl DiscoveryService {
    pub fn new(config: NetworkConfig, device_id: String, device_name: String) -> Self {
        Self {
            config,
            peers: HashMap::new(),
            device_id,
            device_name,
        }
    }
    
    pub async fn start(&mut self, peer_updates: mpsc::UnboundedSender<PeerInfo>) -> Result<()> {
        let socket = Arc::new(UdpSocket::bind(format!("0.0.0.0:{}", self.config.discovery_port)).await?);
        
        // Enable broadcast
        socket.set_broadcast(true)?;
        
        log::info!("Discovery service started on port {}", self.config.discovery_port);
        
        // Start heartbeat sender
        let socket_sender = socket.clone();
        let packet = DiscoveryPacket {
            version: "1.0".to_string(),
            device_id: self.device_id.clone(),
            device_name: self.device_name.clone(),
            tcp_port: self.config.tcp_port,
            capabilities: vec!["text".to_string(), "image".to_string(), "file".to_string()],
        };
        let broadcast_addr: SocketAddr = format!("{}:{}", self.config.multicast_addr, self.config.discovery_port).parse()?;
        let heartbeat_interval = self.config.heartbeat_interval;
        
        tokio::spawn(async move {
            let mut timer = interval(Duration::from_secs(heartbeat_interval));
            loop {
                timer.tick().await;
                
                if let Ok(data) = serde_json::to_vec(&packet) {
                    if let Err(e) = socket_sender.send_to(&data, broadcast_addr).await {
                        log::error!("Failed to send discovery broadcast: {}", e);
                    }
                }
            }
        });
        
        // Start receiver
        let mut buffer = [0u8; 1024];
        loop {
            match socket.recv_from(&mut buffer).await {
                Ok((len, addr)) => {
                    if let Ok(packet) = serde_json::from_slice::<DiscoveryPacket>(&buffer[..len]) {
                        // Ignore our own broadcasts
                        if packet.device_id != self.device_id {
                            let peer_info = PeerInfo {
                                device_id: packet.device_id.clone(),
                                device_name: packet.device_name,
                                tcp_port: packet.tcp_port,
                                capabilities: packet.capabilities,
                                last_seen: SystemTime::now()
                                    .duration_since(UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs(),
                            };
                            
                            // Update peer list
                            let is_new_peer = !self.peers.contains_key(&packet.device_id);
                            self.peers.insert(packet.device_id.clone(), peer_info.clone());
                            
                            if is_new_peer {
                                log::info!("Discovered new peer: {} ({})", peer_info.device_name, peer_info.device_id);
                                let _ = peer_updates.send(peer_info);
                            }
                        }
                    }
                }
                Err(e) => {
                    log::error!("Error receiving discovery packet: {}", e);
                }
            }
            
            // Cleanup expired peers
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            self.peers.retain(|device_id, peer| {
                let is_expired = now - peer.last_seen > self.config.peer_timeout;
                if is_expired {
                    log::info!("Peer expired: {} ({})", peer.device_name, device_id);
                }
                !is_expired
            });
        }
    }
    
    pub fn get_peers(&self) -> &HashMap<String, PeerInfo> {
        &self.peers
    }
}