//! VRM model loader using bevy_vrm1.

use bevy::prelude::*;

pub struct VrmLoader;

impl VrmLoader {
    pub fn load_from_path(path: &str) -> anyhow::Result<Handle<Scene>> {
        // Use bevy_vrm1 to load VRM 1.0 models
        unimplemented!("VRM loading")
    }
}
