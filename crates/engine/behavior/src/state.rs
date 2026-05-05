//! Character state machine.

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharacterState {
    Idle,
    Walking,
    Running,
    Sitting,
    Dragged,
    Jumping,
}

#[derive(Component)]
pub struct Character {
    pub state: CharacterState,
    pub speed: f32,
}
