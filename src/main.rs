use anyhow::Result;
use clap::{Parser, Subcommand};
use log::info;

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
    let _config = Config::load().await?;
    
    info!("xsync daemon started successfully");
    info!("Use Ctrl+C to stop");
    
    // Keep daemon running
    tokio::signal::ctrl_c().await?;
    info!("Shutting down xsync daemon...");
    
    Ok(())
}
