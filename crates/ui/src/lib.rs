//! Settings UI plugin using Bevy's ECS.

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, _app: &mut App) {
        // Settings overlay using Bevy UI
        // Load/save configuration from disk
    }
}
