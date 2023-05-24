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

    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
};

struct MaterialParams {
    base_color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material_params: MaterialParams;
@group(1) @binding(1)
var<uniform> noise_params: NoiseParams;
@group(1) @binding(2)
var texture: texture_2d<f32>;
@group(1) @binding(3)
var texture_sampler: sampler;

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

    let texture_color = textureSampleLevel(texture, texture_sampler, vertex.uv, 0.);

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

    var z = texture_color[1];
    var n = vec3<f32>(0., 0., 1.);

    if texture_color[0] > 0.0 {

        // terrain height
        z = z_height(p, noise_params) * texture_color[0];
        let z_west = z_height(p_west, noise_params) * texture_color[0];
        let z_east = z_height(p_east, noise_params) * texture_color[0];
        let z_north = z_height(p_north, noise_params) * texture_color[0];
        let z_south = z_height(p_south, noise_params) * texture_color[0];

        // define normals
        let stangent = vec3<f32>(2. * delta, 0., z_east - z_west);
        let ttangent = vec3<f32>(0., 2. * delta, z_north - z_south);

        n = vec3<f32>(cross(stangent, ttangent));
    };

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

    // set material parameters from Terrain Material Uniforms
    //material.perceptual_roughness = 1.0;
    material.base_color = textureSample(texture, texture_sampler, in.uv);
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