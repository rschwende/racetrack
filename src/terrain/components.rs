use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
};

use crate::GlobalState;

#[derive(Debug, Clone, Default, ShaderType)]
pub struct TerrainMaterialParams {
    // terrain parameters
    noise_seed: f32,
    pub frequency_scale: f32,
    pub amplitude_scale: f32,
    pub octaves: u32,
    pub lacunarity: f32,
    pub gain: f32,

    // track parameters
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct TerrainMaterial {
    #[uniform(0)]
    pub params: TerrainMaterialParams,
}

impl TerrainMaterialParams {
    pub fn new(global_state: &ResMut<GlobalState>) -> Self {
        TerrainMaterialParams {
            noise_seed: 1.,
            frequency_scale: global_state.frequency_scale,
            amplitude_scale: global_state.amplitude_scale,
            octaves: global_state.octaves as u32,
            lacunarity: global_state.lacunarity,
            gain: global_state.gain,

            x_min: global_state.x_min,
            x_max: global_state.x_max,
            y_min: global_state.y_min,
            y_max: global_state.y_max,
        }
    }
}

impl Material for TerrainMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/terrain_vertex_shader.wgsl".into()
    }

    // fn fragment_shader() -> ShaderRef {
    //     "shaders/custom_material.wgsl".into()
    // }
}
