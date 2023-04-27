use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}};
use noisy_bevy::fbm_simplex_2d_seeded;

use crate::GlobalState;

pub const Y_SUB_MAX_LEN:f32 = 0.3;
pub const X_SUB_MAX_LEN:f32 = 0.3;
pub const NOISE_SEED: f32 = 1.;
pub const DELTA: f32 = 0.01;

/// spawns a list of track elements
pub fn spawn_terrain_mesh(
    mut global_state: ResMut <GlobalState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

   
    global_state.x_max = 5.;
    global_state.x_min = -5.;
    global_state.y_max = 5.;
    global_state.y_min = -5.;

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    terrain_mesh(global_state, &mut mesh);

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    // lighting
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 40.0),
        ..default()
    });

    // // test cube
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
    //     ..Default::default()
    // });

}


/// creates track element mesh in passed mesh and passed transform ends as start position of
/// next track element
pub fn terrain_mesh(
    mut global_state: ResMut <GlobalState>,
    mut mesh: &mut Mesh
) -> bool {
    
    // vectors that define mesh
    let mut indices = vec![];
    let mut positions = vec![]; 
    let mut normals = vec![];
    let mut texture = vec![];
    
    // mesh density
    let num_y_nodes = ((global_state.y_max - global_state.y_min) / Y_SUB_MAX_LEN).ceil() as u32;
    let num_x_nodes = ((global_state.x_max - global_state.x_min) / X_SUB_MAX_LEN).ceil() as u32;

    // define vertices
    for y_node in 0..num_y_nodes {

        let y = y_node as f32 / num_y_nodes as f32 * (global_state.y_max - global_state.y_min) + global_state.y_min;
        let y_north = y + DELTA;
        let y_south = y - DELTA;

        for x_node in 0..num_x_nodes {

            let x = x_node as f32 / num_x_nodes as f32 * (global_state.x_max - global_state.x_min) + global_state.x_min;
            let x_west = x - DELTA;
            let x_east = x + DELTA;

            // X & Y positions for noise
            let p = Vec2::new(x, y);
            let p_west = Vec2::new(x_west, y);
            let p_east = Vec2::new(x_east, y);
            let p_north = Vec2::new(x, y_north);
            let p_south = Vec2::new(x, y_south);

            // Z height from noise
            let z = fbm_simplex_2d_seeded(p * global_state.frequency_scale, 
                                                global_state.octaves, 
                                                global_state.lacunarity, 
                                                global_state.gain, 
                                                NOISE_SEED) * global_state.amplitude_scale;
            let z_west = fbm_simplex_2d_seeded(p_west * global_state.frequency_scale, 
                                                global_state.octaves, 
                                                global_state.lacunarity, 
                                                global_state.gain, 
                                                NOISE_SEED) * global_state.amplitude_scale;
            let z_east = fbm_simplex_2d_seeded(p_east * global_state.frequency_scale, 
                                                global_state.octaves, 
                                                global_state.lacunarity, 
                                                global_state.gain, 
                                                NOISE_SEED) * global_state.amplitude_scale;
            let z_north = fbm_simplex_2d_seeded(p_north * global_state.frequency_scale, 
                                                global_state.octaves, 
                                                global_state.lacunarity, 
                                                global_state.gain, 
                                                NOISE_SEED) * global_state.amplitude_scale;
            let z_south = fbm_simplex_2d_seeded(p_south * global_state.frequency_scale, 
                                                global_state.octaves, 
                                                global_state.lacunarity, 
                                                global_state.gain, 
                                                NOISE_SEED) * global_state.amplitude_scale;

            // define position
            let p = Vec3::new(x, y, z);

            // define normals
            let stangent = Vec3::new(2.*DELTA*(global_state.x_max - global_state.x_min), 0., z_east - z_west);
            let ttangent = Vec3::new(0., 2. * DELTA * (global_state.y_max - global_state.y_min), z_north - z_south);

            let n = Vec3::cross(stangent, ttangent);

            // define texture coordinates
            let uv = Vec2::new(x_node as f32 / num_x_nodes as f32, y_node as f32 / num_y_nodes as f32);

            positions.push(p);
            normals.push(n);
            texture.push(uv);

            // stopped here ------------------------------------------------------------------------------

            // define indices
            if y_node < (num_y_nodes - 1) && x_node < (num_x_nodes - 1) {

                // first triangle
                indices.push((y_node + 1) * num_x_nodes + x_node - num_x_nodes);
                indices.push((y_node + 1) * num_x_nodes + x_node + 1 - num_x_nodes);
                indices.push((y_node + 1) * num_x_nodes + x_node + 1);

                // second triangle
                indices.push((y_node + 1) * num_x_nodes + x_node - num_x_nodes);
                indices.push((y_node + 1) * num_x_nodes + x_node + 1);
                indices.push((y_node + 1) * num_x_nodes + x_node);

            }

        }

    }
    
    // define mesh
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, texture);
    true //return

}

