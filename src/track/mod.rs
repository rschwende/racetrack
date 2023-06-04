use bevy::prelude::*;

pub mod systems;

use systems::*;

pub struct TrackPlugin;

impl Plugin for TrackPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(default_track_list)
            .add_startup_system(setup);
    }
}
