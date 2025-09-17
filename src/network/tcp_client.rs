// TCP client for outgoing connections

use crate::network::{Message, PeerInfo};
use anyhow::Result;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::time::timeout;

pub struct TcpClient;

impl TcpClient {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn send_message(&self, peer: &PeerInfo, message: &Message) -> Result<()> {
        // Connect to peer with timeout
        let addr = format!("{}:{}", "127.0.0.1", peer.tcp_port); // TODO: Get actual IP from discovery
        let stream = timeout(Duration::from_secs(5), TcpStream::connect(&addr)).await??;
        
        let (reader, mut writer) = stream.into_split();
        let mut buf_reader = BufReader::new(reader);
        
        // Send message
        let message_json = serde_json::to_string(message)?;
        writer.write_all(format!("{}\n", message_json).as_bytes()).await?;
        
        // Wait for acknowledgment with timeout
        let mut response = String::new();
        match timeout(Duration::from_secs(5), buf_reader.read_line(&mut response)).await {
            Ok(Ok(_)) => {
                log::debug!("Received response: {}", response.trim());
                Ok(())
            }
            Ok(Err(e)) => {
                log::error!("Error reading response: {}", e);
                Err(e.into())
            }
            Err(_) => {
                log::error!("Timeout waiting for response from peer {}", peer.device_id);
                Err(anyhow::anyhow!("Timeout waiting for response"))
            }
        }
    }
    
    pub async fn send_message_to_address(&self, address: &str, message: &Message) -> Result<()> {
        // Connect to specific address with timeout
        let stream = timeout(Duration::from_secs(5), TcpStream::connect(address)).await??;
        
        let (reader, mut writer) = stream.into_split();
        let mut buf_reader = BufReader::new(reader);
        
        // Send message
        let message_json = serde_json::to_string(message)?;
        writer.write_all(format!("{}\n", message_json).as_bytes()).await?;
        
        // Wait for acknowledgment with timeout
        let mut response = String::new();
        match timeout(Duration::from_secs(5), buf_reader.read_line(&mut response)).await {
            Ok(Ok(_)) => {
                log::debug!("Received response: {}", response.trim());
                Ok(())
            }
            Ok(Err(e)) => {
                log::error!("Error reading response: {}", e);
                Err(e.into())
            }
            Err(_) => {
                log::error!("Timeout waiting for response from {}", address);
                Err(anyhow::anyhow!("Timeout waiting for response"))
            }
        }
    }
}