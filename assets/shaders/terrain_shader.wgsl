// Same imports as <https://github.com/bevyengine/bevy/blob/main/crates/bevy_pbr/src/render/pbr.wgsl>
// vertex shader
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_functions
#import bevy_pbr::mesh_bindings

// fragment shader
#import bevy_pbr::pbr_bindings
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::pbr_ambient
#import bevy_pbr::shadows
#import bevy_pbr::fog
#import bevy_pbr::pbr_functions

#import noisy_bevy::prelude

struct NoiseParams {
    noise_seed: f32,
    frequency_scale: f32,
    amplitude_scale: f32,
    octaves: i32,
    lacunarity: f32,
    gain: f32,
    scale: f32,

    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    max_track_ht: f32,
    min_track_ht: f32,
    show_track_map: i32,
    track_texture_scale: f32,
    terrain_texture_scale: f32,
    };

struct MaterialParams {
    base_color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material_params: MaterialParams;
@group(1) @binding(1)
var<uniform> noise_params: NoiseParams;
@group(1) @binding(2)
var track_map: texture_2d<f32>;
@group(1) @binding(3)
var track_map_sampler: sampler;
@group(1) @binding(4)
var track_texture: texture_2d<f32>;
@group(1) @binding(5)
var track_sampler: sampler;

// vertex structs copied from bevy_pbr/src/render/mesh.wgsl
struct Vertex {
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_NORMALS
    @location(1) normal: vec3<f32>,
#endif
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef SKINNED
    @location(5) joint_indices: vec4<u32>,
    @location(6) joint_weights: vec4<f32>,
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

// fragment struct copied from bevy_pbr/src/render/pbr.wgsl
struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}

fn z_height(p: vec2<f32>, noise_params: NoiseParams) -> f32 {
    let z = fbm_simplex_2d_seeded(p * noise_params.frequency_scale, noise_params.octaves, noise_params.lacunarity, noise_params.gain, noise_params.noise_seed) * noise_params.amplitude_scale;
    return z;
}



@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    // deltas
    let xy_delta = 0.01;
    let u_delta = xy_delta / (noise_params.x_max - noise_params.x_min);
    let v_delta = xy_delta / (noise_params.y_max - noise_params.y_min);

    // xy offsets
    let y_north = vertex.position.y + xy_delta;
    let y_south = vertex.position.y - xy_delta;

    let x_west = vertex.position.x - xy_delta;
    let x_east = vertex.position.x + xy_delta;

    // uv offsets
    let v_north = vertex.uv.y + v_delta; 
    let v_south = vertex.uv.y - v_delta;
    let u_east = vertex.uv.x + u_delta;
    let u_west = vertex.uv.x - u_delta;

    // uv
    let uv_north = vec2<f32>(vertex.uv.x, v_north);
    let uv_south = vec2<f32>(vertex.uv.x, v_south);
    let uv_east = vec2<f32>(u_east, vertex.uv.y);
    let uv_west = vec2<f32>(u_west, vertex.uv.y);

    // texture lookups
    // let texture_color = textureSampleLevel(track_map, track_map_sampler, vertex.uv, 0.);
    // let texture_color_north = textureSampleLevel(track_map, track_map_sampler, uv_north, 0.);
    // let texture_color_south = textureSampleLevel(track_map, track_map_sampler, uv_south, 0.);
    // let texture_color_east = textureSampleLevel(track_map, track_map_sampler, uv_east, 0.);
    // let texture_color_west = textureSampleLevel(track_map, track_map_sampler, uv_west, 0.);

    let texture_color = textureSampleLevel(track_map, track_map_sampler, vertex.uv, 1.);
    let texture_color_north = textureSampleLevel(track_map, track_map_sampler, uv_north, 1.);
    let texture_color_south = textureSampleLevel(track_map, track_map_sampler, uv_south, 1.);
    let texture_color_east = textureSampleLevel(track_map, track_map_sampler, uv_east, 1.);
    let texture_color_west = textureSampleLevel(track_map, track_map_sampler, uv_west, 1.);

    // X & Y positions for noise
    let p = vec2<f32>(vertex.position.x, vertex.position.y);
    let p_west = vec2<f32>(x_west, vertex.position.y);
    let p_east = vec2<f32>(x_east, vertex.position.y);
    let p_north = vec2<f32>(vertex.position.x, y_north);
    let p_south = vec2<f32>(vertex.position.x, y_south);

    // z height from track height
    let z_track = (texture_color[1] * (noise_params.max_track_ht - noise_params.min_track_ht) + noise_params.min_track_ht) * (1. - texture_color[0]);
    let z_track_north = (texture_color_north[1] * (noise_params.max_track_ht - noise_params.min_track_ht) + noise_params.min_track_ht) * (1. - texture_color[0]);
    let z_track_south = (texture_color_south[1] * (noise_params.max_track_ht - noise_params.min_track_ht) + noise_params.min_track_ht) * (1. - texture_color[0]);
    let z_track_east = (texture_color_east[1] * (noise_params.max_track_ht - noise_params.min_track_ht) + noise_params.min_track_ht) * (1. - texture_color[0]);
    let z_track_west = (texture_color_west[1] * (noise_params.max_track_ht - noise_params.min_track_ht) + noise_params.min_track_ht) * (1. - texture_color[0]);

    // let z_track_north = z_track;
    // let z_track_south = z_track;
    // let z_track_east = z_track;
    // let z_track_west = z_track;

    // z height from noise
    let z_terrain = z_height(p, noise_params) * texture_color[0];
    let z_terrain_north = z_height(p_north, noise_params) * texture_color[0];
    let z_terrain_south = z_height(p_south, noise_params) * texture_color[0];
    let z_terrain_east = z_height(p_east, noise_params) * texture_color[0];
    let z_terrain_west = z_height(p_west, noise_params) * texture_color[0];

    // z total height

    let z = z_track + z_terrain;
    let z_north = z_track_north + z_terrain_north;
    let z_south = z_track_south + z_terrain_south;
    let z_east = z_track_east + z_terrain_east;
    let z_west = z_track_west + z_terrain_west;



    // define normals
    let stangent = vec3<f32>(2. * xy_delta, 0., z_east - z_west);
    let ttangent = vec3<f32>(0., 2. * xy_delta, z_north - z_south);

    let n = vec3<f32>(cross(stangent, ttangent));

    // adapted from bevy_pbr/src/render/mesh.wgsl
    #ifdef SKINNED
    var model = skin_model(vertex.joint_indices, vertex.joint_weights);
    #else
    var model = mesh.model;
    #endif

    #ifdef VERTEX_NORMALS
    #ifdef SKINNED
    out.world_normal = skin_normals(model, vertex.normal);
    #else
    out.world_normal = mesh_normal_local_to_world(n);
    #endif
    #endif

    #ifdef VERTEX_POSITIONS
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position.x, vertex.position.y, z, 1.0));
    out.clip_position = mesh_position_world_to_clip(out.world_position);
    #endif

    #ifdef VERTEX_UVS
    out.uv = vertex.uv;
    #endif

    #ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
    #endif

    #ifdef VERTEX_COLORS
    out.color = vertex.color;
    #endif

    return out;
}


// // vertex shader copied from bevy_pbr/src/render/mesh.wgsl
// @vertex
// fn vertex(vertex: Vertex) -> VertexOutput {
//     var out: VertexOutput;

// #ifdef SKINNED
//     var model = skin_model(vertex.joint_indices, vertex.joint_weights);
// #else
//     var model = mesh.model;
// #endif

// #ifdef VERTEX_NORMALS
// #ifdef SKINNED
//     out.world_normal = skin_normals(model, vertex.normal);
// #else
//     out.world_normal = mesh_normal_local_to_world(vertex.normal);
// #endif
// #endif

// #ifdef VERTEX_POSITIONS
//     out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
//     out.clip_position = mesh_position_world_to_clip(out.world_position);
// #endif

// #ifdef VERTEX_UVS
//     out.uv = vertex.uv;
// #endif

// #ifdef VERTEX_TANGENTS
//     out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
// #endif

// #ifdef VERTEX_COLORS
//     out.color = vertex.color;
// #endif

//     return out;
// }

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {

    // bring in params from Terrain Material to Standard Material in order
    // to use pbr shader pipeline
    var material = standard_material_new();

    // textures
    let track_texture_uv = in.uv * vec2<f32>(noise_params.track_texture_scale, noise_params.track_texture_scale);
    let map_color = textureSample(track_map, track_map_sampler, in.uv);
    let track_color = textureSample(track_texture, track_sampler, track_texture_uv);


    // set material parameters from Terrain Material Uniforms
    if noise_params.show_track_map == 1 {
        material.base_color = map_color;
    } else {
        if map_color[0] < 0.01 {
            material.base_color = track_color;
        } else {};
    }


    //material.perceptual_roughness = 1.0;
    //material.base_color = track_color;
    //let baseColor = textureSample(texture, texture_sampler, in.uv);
    //material.unlit = false;

    //material.base_color = vec4<f32>(in.uv[0], in.uv[1], 0., 1.);

    if (in.uv[0] > 1.) {
        material.base_color = vec4<f32>(0., 0., 0., 1.);
    }

    if (in.uv[1] > 1.) {
        material.base_color = vec4<f32>(0., 0., 0., 1.);
    }


    var pbr_input = pbr_input_new();
    pbr_input.frag_coord = in.frag_coord;
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;
    pbr_input.material = material;
    pbr_input.world_normal = prepare_world_normal(
        in.world_normal,
        (material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u,
        in.is_front,
    );

    pbr_input.is_orthographic = view.projection[3].w == 1.0;

    pbr_input.N = apply_normal_mapping(
        material.flags,
        pbr_input.world_normal,
        in.uv,
    );
    pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);

    var output_color = pbr(pbr_input);

    output_color = tone_mapping(output_color);
    var output_rgb = output_color.rgb;
    output_rgb = pow(output_rgb, vec3<f32>(1.0 / 2.2));
    output_rgb = output_rgb + screen_space_dither(in.frag_coord.xy);
    // This conversion back to linear space is required because our output texture format is
    // SRGB; the GPU will assume our output is linear and will apply an SRGB conversion.
    output_rgb = pow(output_rgb, vec3<f32>(2.2));
    output_color = vec4(output_rgb, output_color.a);

    return output_color;
}