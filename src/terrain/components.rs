use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        extract_component::ExtractComponent,
        render_resource::{AsBindGroup, ShaderRef, ShaderType},
    },
};

use crate::GlobalResource;

// noise parameters
#[derive(Component, Clone, ExtractComponent, ShaderType)]
pub struct NoiseParams {
    pub noise_seed: f32,
    pub frequency_scale: f32,
    pub amplitude_scale: f32,
    pub octaves: u32,
    pub lacunarity: f32,
    pub gain: f32,
    pub scale: f32,

    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub max_track_ht: f32,
    pub min_track_ht: f32,
    pub show_track_map: u32,
    pub track_texture_scale: f32,
    pub terrain_texture_scale: f32,
}

impl NoiseParams {
    // constructor
    pub fn new(global_resource: &ResMut<GlobalResource>) -> NoiseParams {
        NoiseParams {
            noise_seed: 1.,
            frequency_scale: global_resource.frequency_scale,
            amplitude_scale: global_resource.amplitude_scale,
            octaves: global_resource.octaves as u32,
            lacunarity: global_resource.lacunarity,
            gain: global_resource.gain,
            scale: global_resource.scale,

            x_min: global_resource.x_min,
            x_max: global_resource.x_max,
            y_min: global_resource.y_min,
            y_max: global_resource.y_max,
            max_track_ht: global_resource.max_track_ht,
            min_track_ht: global_resource.min_track_ht,
            show_track_map: global_resource.show_track_map as u32,
            track_texture_scale: global_resource.track_texture_scale,
            terrain_texture_scale: global_resource.terrain_texture_scale,
        }
    }

    // // method to update struct to current global state
    // pub fn update(global_resource: &ResMut<GlobalResource>) {
    //     NoiseParams {
    //         noise_seed: 1.,
    //         frequency_scale: global_resource.frequency_scale,
    //         amplitude_scale: global_resource.amplitude_scale,
    //         octaves: global_resource.octaves as u32,
    //         lacunarity: global_resource.lacunarity,
    //         gain: global_resource.gain,

    //         x_min: global_resource.x_min,
    //         x_max: global_resource.x_max,
    //         y_min: global_resource.y_min,
    //         y_max: global_resource.y_max,
    //     };
    // }
}

// material parameters
#[derive(Component, Clone, ExtractComponent, ShaderType)]
pub struct MaterialParams {
    pub base_color: Color,
    //pub base_color_texture: Option<Handle<Image>>,
    //pub perceptional_roughness: f32,
    //pub metallic: u32,
}

impl MaterialParams {
    // constructor
    pub fn new() -> MaterialParams {
        MaterialParams {
            base_color: Color::rgb(1.0, 0.5, 0.5),
        }
    }
}

// Material that will be used on terrain plane
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct TerrainMaterial {
    #[uniform(0)]
    pub material_params: MaterialParams,
    #[uniform(1)]
    pub noise_params: NoiseParams,
    #[texture(2)]
    #[sampler(3)]
    pub track_image: Handle<Image>,
    #[texture(4)]
    #[sampler(5)]
    pub track_texture: Handle<Image>,
}

impl Material for TerrainMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/terrain_shader.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/terrain_shader.wgsl".into()
    }
}
