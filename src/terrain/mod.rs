use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use noisy_bevy::NoisyShaderPlugin;
use systems::*;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<TerrainMaterial>::default())
            .add_plugin(NoisyShaderPlugin)
            .add_startup_system(setup)
            .add_system(update_noise_params);
    }
}
