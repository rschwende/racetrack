use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct GlobalState {
    // terrain parameters
    pub frequency_scale: f32,
    pub amplitude_scale: f32,
    pub octaves: usize,
    pub lacunarity: f32,
    pub gain: f32,

    // track parameters
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}
