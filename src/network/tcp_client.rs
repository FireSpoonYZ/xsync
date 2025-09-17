// TCP client for outgoing connections

use crate::network::{Message, PeerInfo};
use anyhow::Result;
use tokio::net::TcpStream;

pub struct TcpClient;

impl TcpClient {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn send_message(&self, _peer: &PeerInfo, _message: &Message) -> Result<()> {
        let _stream = TcpStream::connect(format!("{}:{}", "127.0.0.1", _peer.tcp_port)).await?;
        // TODO: Send message to peer
        Ok(())
    }
}