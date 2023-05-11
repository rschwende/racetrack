use bevy::prelude::*;

mod systems;

use systems::*;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_axis);
    }
}