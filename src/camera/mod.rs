use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera)
        .add_system(pan_orbit_camera);
    }
}