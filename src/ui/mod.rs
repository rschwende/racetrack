use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub mod components;
mod systems;

use systems::*;
use components::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
        .init_resource::<OccupiedScreenSpace>()
        .init_resource::<UiState>()
        //.add_startup_system(setup_system)
        .add_startup_system(configure_ui_state_system)
        .add_system(ui_example_system);
        //.add_system(update_camera_transform_system);
    }
}