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
    pub scale: f32,

    // track parameters
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
    pub max_track_ht: f32,
    pub min_track_ht: f32,

    pub show_track_map: bool,
    pub show_track: bool,
    pub show_track_texture: bool,
    pub show_terrain: bool,
    pub track_texture_scale: f32,
    pub terrain_texture_scale: f32,
}

#[derive(Resource, Default)]
pub struct TrackResource {
    pub track_list: Vec<TrackElement2D>,
    pub track_map_image_handle: Handle<Image>,
    pub track_texture_handle: Handle<Image>,
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
    pub start_bank_angle: f32,
    pub end_bank_angle: f32,
    pub width: f32,
    pub length: f32,
}

// labels
#[derive(Component)]
pub struct RenderToTexturePass;

#[derive(Component)]
pub struct TrackElement;

#[derive(Component)]
pub struct PlaneElement;

#[derive(Component)]
pub struct TerrainElement;
