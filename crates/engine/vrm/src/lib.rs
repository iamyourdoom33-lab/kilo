//! VRM loader plugin integrating bevy_vrm1.

use bevy::prelude::*;

pub struct VrmPlugin;

impl Plugin for VrmPlugin {
    fn build(&self, _app: &mut App) {
        // Load VRM models from assets/models/
        // Register VRM components and systems
    }
}
