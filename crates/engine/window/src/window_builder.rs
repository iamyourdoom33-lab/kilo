//! Window builder for transparent overlay window.

use bevy::window::Window;

pub fn create_transparent_window() -> Window {
    Window {
        title: "Desktop Friend".into(),
        resolution: (1024.0, 768.0).into(),
        // These will be configured via winit in implementation
        transparent: true,
        decorations: false,
        ..default()
    }
}
