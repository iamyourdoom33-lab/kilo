# Desktop Friend - Technical Architecture Document

## Executive Summary

**Recommended Tech Stack: Rust + Bevy**

After analyzing existing projects and available technologies, Rust with the Bevy game engine is the optimal choice for Desktop Friend. This stack provides:
- Native performance with minimal resource usage
- Excellent cross-platform desktop support via winit
- Mature VRM 1.0 support via bevy_vrm1
- Built-in transparent window support
- Strong ecosystem for 3D rendering and animations

---

## Comparative Analysis

### Option 1: Rust + Bevy (RECOMMENDED)

**Pros:**
- Native performance, minimal memory/CPU footprint (~50-200MB RAM for VRM models)
- Direct VRM 1.0 support via `bevy_vrm1` crate (spring bones, look-at, animations)
- Cross-platform transparent windows via winit (Windows/macOS/Linux)
- Built-in 3D rendering, animations, and ECS architecture
- Single binary deployment, no runtime dependencies
- Strong typing and memory safety

**Cons:**
- Smaller ecosystem than Electron/Three.js
- Learning curve for ECS if team isn't familiar
- Linux support via winit has some Wayland limitations

**Existing Projects Using This Stack:**
- Desktop Homunculus - cross-platform desktop mascot with VRM support (macOS, Windows with NVIDIA config)
- MateEngine - desktop companion with VRM models (Steam)

### Option 2: Electron + Three.js

**Pros:**
- Largest ecosystem, extensive documentation
- @pixiv/three-vrm provides mature VRM support
- Web developers can contribute easily
- Rapid prototyping

**Cons:**
- High memory usage (>500MB baseline)
- Requires Chromium runtime
- Performance limitations for smooth animations
- Click-through windows require platform-specific workarounds
- Windows transparency has historically had issues with certain GPU configurations

### Option 3: C++/Qt

**Pros:**
- Native performance
- Qt's graphics capabilities via QGraphicsView/QML
- Cross-platform

**Cons:**
- No mature VRM support libraries
- Manual shader/VRM implementation required
- Longer development time

---

## Recommended Architecture

### Project Structure

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

### Core Dependencies

```toml
[dependencies]
# Core engine
bevy = "0.15"

# VRM Support
bevy_vrm1 = "0.6"  # VRM 1.0 model and animation support

# Window Management
winit = "0.30"  # Cross-platform window creation

# Serialization
serde = { version = "1.0", features = ["derive"] }
ron = "0.8"  # Human-readable config files

# Audio
rodio = "0.17"  # Audio playback

# Platform-specific
[target.'cfg(target_os = "windows")'.dependencies]
winapi = { version = "0.3", features = ["winuser", "shellscalingapi"] }

[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.26"
objc = "0.2"

[target.'cfg(all(unix, not(target_os = "macos")))'.dependencies]
x11 = "2.21"  # For X11 window detection
```

---

## System Components

### 1. Window Management System

**Requirements:**
- Transparent, borderless, always-on-top window
- Click-through mode for desktop interaction
- Multi-monitor support with per-monitor tracking

**Implementation:**
- Use `winit` with transparent window attributes
- Enable `set_cursor_hittest(false)` for click-through (macOS)
- Use `WS_EX_TRANSPARENT` on Windows for click-through
- X11: Use `input_only` or shape extension for click-through

```rust
// Window configuration example
WindowBuilder::new()
    .with_transparent(true)
    .with_always_on_top(true)
    .with_decorations(false)
    .with_window_level(WindowLevel::AlwaysOnTop)
```

### 2. VRM Rendering System

**Features via `bevy_vrm1`:**
- VRM 1.0 model loading
- Humanoid bone mapping (hips, spine, chest, neck, head, arms, legs)
- Spring bones (hair, cloth physics)
- Expression blend shapes (joy, anger, sadness, surprise)
- Look-at constraint for head/eye tracking
- VRMA animation support

**Animation States:**
- Idle (looping)
- Walking
- Running
- Jumping
- Sitting (on windows or desktop)
- Drag-follow
- Head/eye tracking toward cursor

### 3. Window Capture System

**Platform APIs:**
- **Windows:** Win32 `EnumWindows` + `GetWindowText`/`GetWindowRect`
- **macOS:** Accessibility API (`AXUIElement`)
- **Linux X11:** `XQueryTree`, `_NET_CLIENT_LIST`
- **Linux Wayland:** `zwlr_foreign_toplevel_management_v1`

**Features:**
- Track window positions, sizes, and Z-order
- Detect window state (minimized, maximized)
- Allow characters to "sit" on window borders
- Handle dynamic window changes (resize, close, open)

### 4. Behavior and Animation Controller

**State Machine:**
```rust
enum CharacterState {
    Idle { idle_type: IdleType },
    Walking { target: Vec2 },
    Running { direction: Vec2 },
    Sitting { surface: SurfaceType },
    Dragged { offset: Vec2 },
}
```

**Interactions:**
- Mouse proximity detection
- Click to pick up/drag
- Cursor following with smooth interpolation
- Boundary checking to keep on screen
- Physics-based movement (velocity, acceleration)

### 5. Audio System

- WAV/OGG playback via `rodio`
- Sound effects for interactions
- Voice playback for notifications
- Optional mute per-character

---

## Platform-Specific Considerations

### Windows
- Require NVIDIA GPU configuration for transparent windows (known issue)
- Use `WS_EX_LAYERED` for transparency
- Win32 API for window enumeration

### macOS
- Use `NSWindow` with transparent background
- Accessibility permission for window detection
- `setIgnoresMouseEvents(true)` for click-through

### Linux
- Wayland: Limited overlay support (layer-shell protocol)
- X11: Better support via `XComposite` and `XFixes`
- GTK4 layer-shell as alternative for overlay mode

---

## Feature Implementation Roadmap

### Phase 1: Core Foundation
1. Transparent window with click-through
2. Basic 3D scene with placeholder geometry
3. GLM model loading
4. Character rendering with basic animations

### Phase 2: VRM Support
1. Integrate `bevy_vrm1` for VRM 1.0
2. Spring bone physics
3. Expression blend shapes
4. Look-at tracking

### Phase 3: Desktop Integration
1. Window detection and tracking
2. Character sitting on windows
3. Multi-monitor support
4. Drag-and-drop interaction

### Phase 4: Features
1. Alarm/timer system
2. Settings menu (egui)
3. Mod support for custom assets
4. Audio playback

---

## Performance Targets

- Memory: < 200MB (single character)
- CPU: < 5% on modern hardware (idle)
- GPU: Low-end integrated graphics capable
- FPS: 30-60 adjustable (default 30 for battery life)

---

## Security Considerations

- VRM files loaded from local filesystem only (no remote loading)
- Audio files from local filesystem only
- No network access required (VRMs contain no executable code)
- Mods limited to asset replacement, sandboxed environment