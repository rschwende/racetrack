pub mod camera;
pub mod track;

use camera::CameraPlugin;
use track::TrackPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(CameraPlugin)
        .add_plugin(TrackPlugin)
        .run();
}
