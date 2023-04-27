pub mod camera;
pub mod track;
pub mod tools;
pub mod terrain;
pub mod ui;

pub mod components;
mod systems;

use systems::*;
use components::*;

use camera::CameraPlugin;
use track::TrackPlugin;
use tools::ToolsPlugin;
use terrain::TerrainPlugin;
use ui::UIPlugin;

use bevy::prelude::*;
// need to work on this: use bevy_fly_camera::{FlyCamera, FlyCameraPlugin};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)


        .init_resource::<GlobalState>()


        .add_plugin(CameraPlugin)
        //.add_plugin(TrackPlugin)
        //.add_plugin(ToolsPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(UIPlugin)
        .run();
}
