//! Drag-and-drop interaction system.

use bevy::prelude::*;

#[derive(Component)]
pub struct Draggable {
    pub is_dragged: bool,
    pub drag_offset: Vec2,
}

pub struct DragPlugin;

impl Plugin for DragPlugin {
    fn build(&self, _app: &mut App) {
        // Mouse down - pick up character
        // Mouse drag - move character
        // Mouse up - release character
    }
}
