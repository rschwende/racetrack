pub mod camera;
pub mod terrain;
pub mod tools;
pub mod track;
pub mod ui;

pub mod components;
mod systems;

use components::*;
use systems::*;

use camera::CameraPlugin;
use terrain::TerrainPlugin;
//use tools::ToolsPlugin;
use track::TrackPlugin;
use ui::UIPlugin;

use bevy::prelude::*;
// need to work on this: use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GlobalState>()
        .add_plugin(CameraPlugin)
        .add_plugin(TrackPlugin)
        //.add_plugin(ToolsPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(set_global_state)
        .run();
}
