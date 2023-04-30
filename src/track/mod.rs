use bevy::prelude::*;

pub mod components;
mod systems;

use systems::*;

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_track_element);
    }
}
