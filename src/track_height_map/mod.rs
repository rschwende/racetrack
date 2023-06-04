use bevy::prelude::*;

pub mod components;
pub mod systems;

use components::*;
use systems::*;

pub struct TrackHeightMapPlugin;

impl Plugin for TrackHeightMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<TextureMaterial>::default())
            .add_startup_system(setup);
    }
}
