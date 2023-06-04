// adapted from bevy_egui example: https://github.com/mvlabat/bevy_egui/blob/v0.20.1/examples/side_panel.rs

use bevy::prelude::*;

use crate::components::TrackElement2D;

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Resource, Deref, DerefMut)]
pub struct OriginalCameraTransform(pub Transform);

#[derive(Resource, Default)]
pub struct UIResource {
    pub new_track_element: TrackElement2D,
    pub new_index: usize,
}
