//! Global mouse tracking system.

use bevy::prelude::*;

#[derive(Resource)]
pub struct MouseState {
    pub position: Vec2,
    pub normalized_position: Vec2,
    pub screen_bounds: (f32, f32),
}

pub fn update_mouse_position(
    mut mouse: ResMut<MouseState>,
    windows: Query<&Window>,
) {
    // Update mouse position from Bevy input
    unimplemented!("Mouse tracking update")
}
