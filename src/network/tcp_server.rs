// TCP server for incoming connections

use crate::network::{Message, NetworkConfig};
use anyhow::Result;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

pub struct TcpServer {
    config: NetworkConfig,
}

impl TcpServer {
    pub fn new(config: NetworkConfig) -> Self {
        Self { config }
    }
    
    pub async fn start(&self, message_handler: mpsc::UnboundedSender<Message>) -> Result<()> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", self.config.tcp_port)).await?;
        log::info!("TCP server listening on port {}", self.config.tcp_port);
        
        loop {
            match listener.accept().await {
                Ok((socket, addr)) => {
                    log::debug!("New connection from {}", addr);
                    let handler = message_handler.clone();
                    
                    tokio::spawn(async move {
                        if let Err(e) = Self::handle_connection(socket, handler).await {
                            log::error!("Error handling connection from {}: {}", addr, e);
                        }
                    });
                }
                Err(e) => {
                    log::error!("Error accepting connection: {}", e);
                }
            }
        }
    }
    
    async fn handle_connection(mut socket: TcpStream, message_handler: mpsc::UnboundedSender<Message>) -> Result<()> {
        let (reader, mut writer) = socket.split();
        let mut buf_reader = BufReader::new(reader);
        let mut line = String::new();
        
        loop {
            line.clear();
            match buf_reader.read_line(&mut line).await {
                Ok(0) => {
                    // Connection closed
                    break;
                }
                Ok(_) => {
                    // Parse and handle message
                    match serde_json::from_str::<Message>(&line.trim()) {
                        Ok(message) => {
                            log::debug!("Received message: {:?}", message);
                            if let Err(e) = message_handler.send(message) {
                                log::error!("Failed to forward message: {}", e);
                                break;
                            }
                            
                            // Send acknowledgment
                            let ack = serde_json::to_string(&"ACK")?;
                            writer.write_all(format!("{}\n", ack).as_bytes()).await?;
                        }
                        Err(e) => {
                            log::error!("Failed to parse message: {}", e);
                            let error_response = serde_json::to_string(&"ERROR: Invalid message format")?;
                            writer.write_all(format!("{}\n", error_response).as_bytes()).await?;
                        }
                    }
                }
                Err(e) => {
                    log::error!("Error reading from connection: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
}