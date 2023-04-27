use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_terrain_mesh);
    }
}