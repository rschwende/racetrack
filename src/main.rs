pub mod camera;
pub mod terrain;
pub mod tools;
pub mod track;
pub mod track_height_map;
pub mod ui;

pub mod components;
mod systems;

use components::*;
use systems::*;

use camera::CameraPlugin;
use terrain::TerrainPlugin;
use track::TrackPlugin;
use track_height_map::TrackHeightMapPlugin;
use ui::UIPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<GlobalResource>()
        .init_resource::<TrackResource>()
        .init_resource::<MeshResource>()
        .add_startup_system(directional_light)
        .add_startup_system(set_global_resource)
        .add_startup_system(load_assets)
        .add_system(change_texture_mode)
        .add_system(set_visibility)
        .add_plugin(CameraPlugin)
        .add_plugin(TrackPlugin)
        .add_plugin(TrackHeightMapPlugin)
        .add_plugin(TerrainPlugin)
        .add_plugin(UIPlugin)
        .run();
}
