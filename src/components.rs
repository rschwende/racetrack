use std::vec;

use bevy::prelude::*;

pub const TERRAIN_OFFSET: f32 = 25.; // how much the terrain extends past track

#[derive(Resource, Default)]
pub struct GlobalResource {
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

// impl FromWorld for GlobalResource {
//     fn from_world(world: &mut World) -> Self {
//         GlobalResource {
//             frequency_scale: 0.0,
//             amplitude_scale: 0.0,
//             octaves: 0,
//             lacunarity: 0.0,
//             gain: 0.0,
//             x_min: -TERRAIN_OFFSET,
//             x_max: TERRAIN_OFFSET,
//             y_min: -TERRAIN_OFFSET,
//             y_max: TERRAIN_OFFSET,
//         }
//     }
// }

#[derive(Resource, Default)]
pub struct TrackResource {
    pub track_list: Vec<TrackElement2D>,
    pub track_texture_image_handle: Image,
}

#[derive(Resource, Default)]
pub struct MeshResource {
    pub track_mesh_list: Vec<Handle<Mesh>>,
    pub track_mesh_transform_list: Vec<Transform>,
}

#[derive(Component)]
pub struct TrackElement2D {
    pub curvature: f32,
    pub curve_angle: f32,
    pub width: f32,
    pub length: f32,
}

//
#[derive(Component)]
pub struct TrackRenderPass;

//
#[derive(Component)]
pub struct RenderToTexturePass;

#[derive(Component)]
pub struct TestPass;

//
#[derive(Component)]
pub struct TerrainPass;
