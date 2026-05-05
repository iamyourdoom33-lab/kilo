//! Emotion system affecting animation and behavior.

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Emotion {
    Happy,
    Neutral,
    Sad,
    Angry,
    Surprised,
}

#[derive(Component)]
pub struct EmotionalState {
    pub current: Emotion,
    pub intensity: f32, // 0.0 to 1.0
}
