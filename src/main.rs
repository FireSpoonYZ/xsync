use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;
use tokio::sync::mpsc;

mod clipboard;
mod network;
mod storage;
mod sync;

use storage::Config;

#[derive(Parser)]
#[command(name = "xsync")]
#[command(about = "LAN Clipboard Synchronization Application")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the xsync daemon
    Start,
    /// Stop the xsync daemon
    Stop,
    /// Show sync status and connected peers
    Status,
    /// Manually sync content
    Sync {
        #[arg(long)]
        text: Option<String>,
        #[arg(long)]
        file: Option<String>,
    },
    /// Edit configuration
    Config,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Start) => {
            info!("Starting xsync daemon...");
            start_daemon().await?;
        }
        Some(Commands::Stop) => {
            info!("Stopping xsync daemon...");
            // TODO: Implement daemon stop
            println!("Daemon stopped");
        }
        Some(Commands::Status) => {
            info!("Showing sync status...");
            // TODO: Implement status display
            println!("Status: Not implemented yet");
        }
        Some(Commands::Sync { text, file }) => {
            if let Some(text) = text {
                info!("Manually syncing text: {}", text);
                // TODO: Implement manual text sync
            }
            if let Some(file) = file {
                info!("Manually syncing file: {}", file);
                // TODO: Implement manual file sync
            }
        }
        Some(Commands::Config) => {
            info!("Opening configuration...");
            // TODO: Implement config editor
            println!("Config editor not implemented yet");
        }
        None => {
            // Default to start if no command specified
            info!("Starting xsync daemon (default)...");
            start_daemon().await?;
        }
    }
    
    Ok(())
}

async fn start_daemon() -> Result<()> {
    info!("Loading configuration...");
    let config = Config::load().await?;
    
    info!("Initializing clipboard monitor...");
    let (clipboard_tx, mut clipboard_rx) = mpsc::unbounded_channel();
    let _clipboard_monitor = clipboard::ClipboardMonitor::new(clipboard_tx).await?;
    
    info!("Starting network services...");
    let (peer_tx, mut peer_rx) = mpsc::unbounded_channel();
    let (message_tx, mut message_rx) = mpsc::unbounded_channel();
    
    // Start discovery service
    let device_id = uuid::Uuid::new_v4().to_string();
    let mut discovery = network::DiscoveryService::new(
        config.network.clone(),
        device_id.clone(),
        config.device_name.clone(),
    );
    
    tokio::spawn(async move {
        if let Err(e) = discovery.start(peer_tx).await {
            log::error!("Discovery service error: {}", e);
        }
    });
    
    // Start TCP server
    let tcp_server = network::TcpServer::new(config.network.clone());
    tokio::spawn(async move {
        if let Err(e) = tcp_server.start(message_tx).await {
            log::error!("TCP server error: {}", e);
        }
    });
    
    // Initialize sync engine
    let mut sync_engine = sync::SyncEngine::new(config.sync.clone());
    if let Err(e) = sync_engine.start().await {
        log::error!("Failed to start sync engine: {}", e);
        return Err(e);
    }
    
    info!("xsync daemon started successfully");
    info!("Use Ctrl+C to stop");
    
    // Main event loop
    loop {
        tokio::select! {
            // Handle clipboard changes
            Some(clipboard_event) = clipboard_rx.recv() => {
                info!("Clipboard changed: {:?}", clipboard_event.content);
                if let Err(e) = sync_engine.handle_clipboard_change(clipboard_event).await {
                    log::error!("Error handling clipboard change: {}", e);
                }
            }
            
            // Handle new peers
            Some(peer_info) = peer_rx.recv() => {
                info!("New peer discovered: {} ({})", peer_info.device_name, peer_info.device_id);
            }
            
            // Handle incoming messages
            Some(message) = message_rx.recv() => {
                info!("Received message: {:?}", message);
                // TODO: Handle different message types
            }
            
            // Handle Ctrl+C
            _ = tokio::signal::ctrl_c() => {
                info!("Shutting down xsync daemon...");
                break;
            }
        }
    }
    
    Ok(())
}
