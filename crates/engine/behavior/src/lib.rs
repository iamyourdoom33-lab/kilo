//! Character behavior and AI state machine
//!
//! This module implements:
//! - Character state machine (idle, walking, running, sitting, dragging)
//! - Autonomous behavior patterns (roaming, window interaction)
//! - Emotion system affecting animation selection
//! - Decision making for character actions

pub mod state;
pub mod ai;
pub mod emotions;
