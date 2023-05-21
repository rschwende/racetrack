use bevy::prelude::*;

//pub mod components;
mod systems;

use systems::*;

pub struct TrackHeightMapPlugin;

impl Plugin for TrackHeightMapPlugin {
    fn build(&self, app: &mut App) {}
}
