// adapted from bevy_egui example: https://github.com/mvlabat/bevy_egui/blob/v0.20.1/examples/side_panel.rs

use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Resource, Deref, DerefMut)]
pub struct OriginalCameraTransform(pub Transform);

#[derive(Default, Resource)]
pub struct UiState {

    // terrain parameters
    pub frequency_scale: f32,
    pub amplitude_scale: f32,
    pub octaves: f32,
    pub lacunarity: f32,
    pub gain: f32,


    // other
    pub label: String,
    pub value: f32,
    pub is_window_open: bool,
}