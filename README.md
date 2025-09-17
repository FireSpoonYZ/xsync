# xsync - LAN Clipboard Synchronization Application

A peer-to-peer clipboard synchronization application for Local Area Networks (LANs). Share clipboard content (text, images, and files) between multiple devices automatically without requiring a central server.

## Features

- **Cross-platform support**: Linux (with X11) and Windows (planned)
- **Multiple content types**: Plain text, images (PNG, JPEG, BMP), and files
- **Peer-to-peer architecture**: Fully decentralized, no central server required
- **Automatic discovery**: Devices discover each other automatically on the same network
- **Event-driven sync**: Clipboard changes trigger automatic synchronization
- **Conflict resolution**: Timestamp-based conflict resolution
- **Compression**: Built-in compression for efficient data transfer

## Installation

### From Source

1. Ensure you have Rust installed: https://rustup.rs/
2. Clone the repository:
   ```bash
   git clone https://github.com/FireSpoonYZ/xsync.git
   cd xsync
   ```
3. Build the application:
   ```bash
   cargo build --release
   ```
4. The binary will be available at `target/release/xsync`

## Usage

### Start the daemon
```bash
xsync start
# or simply
xsync
```

### CLI Commands
```bash
xsync start              # Start daemon
xsync stop               # Stop daemon
xsync status             # Show peers and sync status
xsync sync --text "..."  # Manually sync text
xsync sync --file path   # Manually sync file
xsync config             # Edit configuration
xsync --help             # Show help
```

### Configuration

The configuration file is automatically created at:
- Linux: `~/.config/xsync/config.toml`
- Windows: `%APPDATA%\xsync\config.toml`

Example configuration:
```toml
device_name = "MyComputer"

[network]
discovery_port = 52525
tcp_port = 52526
multicast_addr = "239.255.255.250"
heartbeat_interval = 5
peer_timeout = 15

[sync]
auto_sync = true
sync_text = true
sync_images = true
sync_files = true
max_file_size = 1073741824  # 1GB

[storage]
cache_dir = "~/.xsync/cache"
receive_dir = "~/Downloads/xsync"
```

## Architecture

### Network Protocol

- **Discovery**: UDP broadcast on port 52525 for peer discovery
- **Data Transfer**: TCP on port 52526 for reliable message transfer
- **Message Types**: SYNC_REQUEST, SYNC_RESPONSE, CLIPBOARD_UPDATE, FILE_REQUEST, FILE_DATA, ACK

### Project Structure

```
src/
├── clipboard/          # Platform-specific clipboard handling
│   ├── monitor.rs      # Clipboard change monitoring
│   ├── reader.rs       # Read clipboard content
│   └── writer.rs       # Write clipboard content
├── network/            # Network communication
│   ├── discovery.rs    # UDP peer discovery
│   ├── tcp_server.rs   # TCP server for incoming connections
│   ├── tcp_client.rs   # TCP client for outgoing connections
│   └── protocol.rs     # Message protocol definitions
├── sync/               # Synchronization logic
│   ├── engine.rs       # Main sync coordination
│   ├── conflict.rs     # Conflict resolution
│   └── state.rs        # State management
├── storage/            # Data storage and caching
│   ├── cache.rs        # Temporary file storage
│   └── config.rs       # Configuration management
└── main.rs             # Application entry point
```

## Requirements

### Linux
- X11 display server (for clipboard access)
- Network access for peer discovery and communication

### Windows (Planned)
- Windows 10/11
- Network access for peer discovery and communication

## Known Limitations

- Currently only supports Linux with X11
- Requires network broadcast permissions for peer discovery
- File transfer size limited by available memory and configuration

## Development Status

This is an active development project. Current implementation includes:
- ✅ Basic project structure and CLI
- ✅ Configuration management
- ✅ Clipboard monitoring (Linux/X11)
- ✅ Network discovery and communication
- ✅ Basic synchronization framework
- 🚧 Complete file transfer implementation
- 🚧 Windows platform support
- 🚧 GUI/System tray interface

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source. License details to be determined.