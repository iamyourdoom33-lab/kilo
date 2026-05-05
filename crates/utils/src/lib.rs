//! Shared utilities and error types.

use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DesktopFriendError {
    #[error("Platform error: {0}")]
    Platform(String),

    #[error("VRM loading error: {0}")]
    Vrm(String),

    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, DesktopFriendError>;
