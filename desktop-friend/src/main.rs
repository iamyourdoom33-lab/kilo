//! Desktop Friend - Main Application Entry Point
//!
//! A cross-platform 3D desktop mascot where animated characters
//! roam on screen, sit on windows, interact with mouse, and provide alarms.

use anyhow::Result;
use bevy::prelude::*;
use env_logger::Env;

mod app {
    pub use bevy::prelude::*;
}

fn main() -> Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    log::info!("Starting Desktop Friend v{}", env!("CARGO_PKG_VERSION"));

    // Build and run Bevy app
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window::Window {
                title: "Desktop Friend".into(),
                resolution: (800.0, 600.0).into(),
                // Transparent, borderless, always on top configured via window plugin
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            // Core engine plugins
            engine::window::WindowPlugin,
            engine::vrm::VrmPlugin,
            engine::behavior::BehaviorPlugin,
            engine::interaction::InteractionPlugin,
            // UI plugin
            ui::UiPlugin,
        ))
        .add_systems(Startup, setup)
        .run();

    Ok(())
}

fn setup(mut commands: Commands) {
    log::info!("Desktop Friend initialized successfully");
    log::info!("Place VRM models in assets/models/ to load them");
}
