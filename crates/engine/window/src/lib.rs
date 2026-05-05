//! Window management plugin for transparent overlay window.

use bevy::prelude::*;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, _app: &mut App) {
        // Configure transparent overlay window
        // Will implement winit window builder with:
        // - transparent: true
        // - decorations: false
        // - always_on_top: true
        // - cursor_hittest: false (click-through)
    }
}
