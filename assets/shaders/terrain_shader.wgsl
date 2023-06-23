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

fn BiLinearSample (vert_cord: vec2<f32>, text_coord: vec2<f32>, noise_params: NoiseParams) -> f32 {

    let w = 2048. - 1.;
    let h = 2048. - 1.;

    let x1 = floor(text_coord.x * w);
    let y1 = floor(text_coord.y * h);

    let x2 = clamp(x1 + 1., 0., w);
    let y2 = clamp(y1 + 1., 0., h);

    let xp = text_coord.x * w - x1;
    let yp = text_coord.y * h - y1;

    let delta = noise_params.max_track_ht - noise_params.min_track_ht;

    let p11_color = textureSampleLevel(track_map, track_map_sampler, vec2<f32>(x1/w, y1/h), 1.);
    let p21_color = textureSampleLevel(track_map, track_map_sampler, vec2<f32>(x2/w, y1/h), 1.);
    let p12_color = textureSampleLevel(track_map, track_map_sampler, vec2<f32>(x1/w, y2/h), 1.);
    let p22_color = textureSampleLevel(track_map, track_map_sampler, vec2<f32>(x2/w, y2/h), 1.);

    // z height from track height
    let p11_track = (p11_color[1] * (noise_params.max_track_ht - noise_params.min_track_ht) + noise_params.min_track_ht) * (1. - p11_color[0]);
    let p21_track = (p21_color[1] * (noise_params.max_track_ht - noise_params.min_track_ht) + noise_params.min_track_ht) * (1. - p21_color[0]);
    let p12_track = (p12_color[1] * (noise_params.max_track_ht - noise_params.min_track_ht) + noise_params.min_track_ht) * (1. - p12_color[0]);
    let p22_track = (p22_color[1] * (noise_params.max_track_ht - noise_params.min_track_ht) + noise_params.min_track_ht) * (1. - p22_color[0]);

    // z height from noise
    let p11_terrain = z_height(vec2<f32>(x1/w, y1/h), noise_params) * p11_color[0];
    let p21_terrain = z_height(vec2<f32>(x2/w, y1/h), noise_params) * p21_color[0];
    let p12_terrain = z_height(vec2<f32>(x1/w, y2/h), noise_params) * p12_color[0];
    let p22_terrain = z_height(vec2<f32>(x2/w, y2/h), noise_params) * p22_color[0];

    // total z
    let p11 = p11_track + p11_terrain;
    let p21 = p21_track + p21_terrain;
    let p12 = p12_track + p12_terrain;
    let p22 = p22_track + p22_terrain;

    let px1 = lerp(xp, p11, p21);
    let px2 = lerp(xp, p12, p22);

    return lerp(yp, px1, px2);
}

fn lerp (x: f32, a: f32, b: f32) -> f32 {
    return x * (b - a) + a;
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

    // X & Y positions for noise
    let p = vec2<f32>(vertex.position.x, vertex.position.y);
    let p_west = vec2<f32>(x_west, vertex.position.y);
    let p_east = vec2<f32>(x_east, vertex.position.y);
    let p_north = vec2<f32>(vertex.position.x, y_north);
    let p_south = vec2<f32>(vertex.position.x, y_south);


    // z height from track height
    let z = BiLinearSample(p, vertex.uv, noise_params);
    let z_north = BiLinearSample(p_north, uv_north, noise_params);
    let z_south = BiLinearSample(p_south, uv_south, noise_params);
    let z_east = BiLinearSample(p_east, uv_east, noise_params);
    let z_west = BiLinearSample(p_west, uv_west, noise_params);

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
    //material.perceptual_roughness = 0.8;
    //material.metallic = 0.0;

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
        } else {
            material.base_color = vec4<f32>(0., 0.3, 0., 1.);
        };
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