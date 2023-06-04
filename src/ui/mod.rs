use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub mod components;
mod systems;

use components::*;
use systems::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .init_resource::<UIResource>()
            .add_startup_system(setup)
            .add_system(ui_system)
            .add_system(track_list_system);
    }
}
