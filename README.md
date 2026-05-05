# Desktop Friend - Free Desktop Mascot App

A cross-platform 3D desktop mascot application where animated characters roam on your screen, sit on windows, interact with your mouse, and provide helpful alarms.

## Features

- **VRM Avatar Support**: Load and animate VRM 1.0 models with full humanoid bone mapping
- **Transparent Overlay Window**: Characters appear on top of all windows without blocking interaction
- **Mouse Interaction**: Characters follow your cursor with smooth, natural movement
- **Window Awareness**: Characters can sit on top of your application windows
- **Alarm & Timer System**: Get friendly reminders from your character with custom animations
- **Cross-Platform**: Works on Windows, macOS, and Linux
- **Mod Support**: Load custom sounds, particles, and behaviors

## Tech Stack

- **Language**: Rust (100%)
- **Game Engine**: Bevy 0.15
- **Window Management**: winit 0.30
- **VRM Support**: bevy_vrm1 0.6
- **Audio**: rodio 0.17
- **Configuration**: serde + RON

## Quick Start

### Prerequisites

- Rust 1.75+ (install via [rustup](https://rustup.rs))
- Git

### Building

```bash
# Clone the repository
git clone https://github.com/your-org/desktop-friend.git
cd desktop-friend

# Build in release mode
cargo build --release

# Run
cargo run --release
```

### Adding VRM Models

Place your `.vrm` files in the `assets/models/` directory. They will be automatically detected at runtime.

## Project Structure

```
desktop-friend/
├── Cargo.toml                 # Workspace configuration
├── crates/
│   ├── engine/                # Core Bevy plugins
│   │   ├── window/            # Transparent overlay window management
│   │   ├── vrm/               # VRM loading and animation system
│   │   ├── behavior/          # Character AI and state machines
│   │   └── interaction/       # Mouse tracking, drag-drop
│   ├── platform/              # Platform-specific code
│   │   ├── windows/           # Win32 window enumeration
│   │   ├── macos/             # macOS accessibility API
│   │   └── linux/             # X11/Wayland window detection
│   ├── ui/                    # Settings and configuration
│   └── utils/                 # Shared utilities
├── assets/
│   ├── shaders/               # WGSL shaders
│   └── default_models/        # Default VRM models
└── tests/
```

## Architecture

See [ARCHITECTURE.md](./ARCHITECTURE.md) for detailed technical documentation including:

- Comparative analysis of tech stacks (Rust/Bevy vs Electron/Three.js vs C++/Qt)
- Detailed component breakdown
- Platform-specific considerations
- Performance targets
- Security model
- Implementation roadmap

## Contributing

This project is in active development. Contributions are welcome!

- Fork the repository
- Create a feature branch
- Submit a pull request

## License

MIT License - see LICENSE file for details.

## Acknowledgments

- [Bevy ECS](https://bevyengine.org/) - Game engine
- [VRM](https://vrm.dev/) - 3D avatar format
- [Desktop Homunculus](https://github.com/aj409/desktop-homunculus) - Inspiration
- [MateEngine](https://store.steampowered.com/app/3202160/MateEngine/) - Reference implementation
