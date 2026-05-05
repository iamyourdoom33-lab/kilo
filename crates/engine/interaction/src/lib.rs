//! Mouse tracking and drag-drop interaction
//!
//! This module handles:
//! - Global mouse cursor position tracking
//! - Click detection and character picking
//! - Drag-and-drop with smooth following
//! - Multi-touch support for multiple characters
//! - Boundary constraints to keep character on-screen

pub mod mouse;
pub mod drag;
pub mod boundary;
