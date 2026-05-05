//! Boundary constraints to keep character on-screen.

use bevy::prelude::*;

#[derive(Component)]
pub struct BoundingBox {
    pub min: Vec2,
    pub max: Vec2,
}

pub fn enforce_boundaries(
    mut query: Query<(&mut Transform, &BoundingBox)>,
) {
    // Keep characters within bounds
    unimplemented!("Boundary enforcement")
}
