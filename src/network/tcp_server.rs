// TCP server for incoming connections

use crate::network::{Message, NetworkConfig};
use anyhow::Result;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub struct TcpServer {
    config: NetworkConfig,
}

impl TcpServer {
    pub fn new(config: NetworkConfig) -> Self {
        Self { config }
    }
    
    pub async fn start(&self, _message_handler: mpsc::UnboundedSender<Message>) -> Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.config.tcp_port)).await?;
        log::info!("TCP server listening on port {}", self.config.tcp_port);
        
        loop {
            let (_socket, _addr) = listener.accept().await?;
            // TODO: Handle incoming connections and parse messages
        }
    }
}