#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::mesh_functions

#import noisy_bevy::prelude

struct TerrainMaterialParams {
    noise_seed: f32,
    frequency_scale: f32,
    amplitude_scale: f32,
    octaves: i32,
    lacunarity: f32,
    gain: f32,

    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
};

@group(1) @binding(0)
var<uniform> params: TerrainMaterialParams;

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    //@location(4) color: vec4<f32>,
};

fn z_height(p: vec2<f32>, params: TerrainMaterialParams) -> f32 {

    let z = fbm_simplex_2d_seeded(p * params.frequency_scale, params.octaves, params.lacunarity, params.gain, params.noise_seed) * params.amplitude_scale;
    return z;
}


@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    let delta = 0.01;

    let y_north = vertex.position.y + delta;
    let y_south = vertex.position.y - delta;

    let x_west = vertex.position.x - delta;
    let x_east = vertex.position.x + delta;

    // X & Y positions for noise
    let p = vec2<f32>(vertex.position.x, vertex.position.y);
    let p_west = vec2<f32>(x_west, vertex.position.y);
    let p_east = vec2<f32>(x_east, vertex.position.y);
    let p_north = vec2<f32>(vertex.position.x, y_north);
    let p_south = vec2<f32>(vertex.position.x, y_south);

    // // Z height from noise
    let z = z_height(p, params);
    let z_west = z_height(p_west, params);
    let z_east = z_height(p_east, params);
    let z_north = z_height(p_north, params);
    let z_south = z_height(p_south, params);

    //define position
    out.world_position = mesh_position_local_to_world(mesh.model, vec4<f32>(vertex.position.x, vertex.position.y, z, 1.0));
    out.clip_position = mesh_position_world_to_clip(out.world_position);

    // define normals
    let stangent = vec3<f32>(2. * delta * (params.x_max - params.x_min), 0., z_east - z_west);
    let ttangent = vec3<f32>(0., 2. * delta * (params.y_max - params.y_min), z_north - z_south);

    let n = vec3<f32>(cross(stangent, ttangent));

    out.world_normal = mesh_normal_local_to_world(n);

    // define texture coordinates
    out.uv = vertex.uv;

    return out;
}