use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::terrain::components::*;
use crate::GlobalState;

pub const Y_SUB_MAX_LEN: f32 = 0.05;
pub const X_SUB_MAX_LEN: f32 = 0.05;
pub const DELTA: f32 = 0.01;

pub fn spawn_terrain(
    mut global_state: ResMut<GlobalState>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut terrain_material: ResMut<Assets<TerrainMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let new_terrain = TerrainMaterial {
        params: TerrainMaterialParams::new(&global_state),
    };

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    terrain_mesh(&global_state, &mut mesh);

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: terrain_material.add(new_terrain),
        ..default()
    });

    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(mesh),
    //     material: materials.add(Color::rgb(1., 0.5, 0.5).into()),
    //     ..default()
    // });

    // lighting
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(0.0, 0.0, 40.0),
    //     ..default()
    // });

    // test cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

pub fn terrain_mesh(global_state: &ResMut<GlobalState>, mesh: &mut Mesh) -> bool {
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
        let y = y_node as f32 / num_y_nodes as f32 * (global_state.y_max - global_state.y_min)
            + global_state.y_min;

        for x_node in 0..num_x_nodes {
            let x = x_node as f32 / num_x_nodes as f32 * (global_state.x_max - global_state.x_min)
                + global_state.x_min;

            // define position
            let p = Vec3::new(x, y, 0.);

            // define normals
            let n = Vec3::new(0., 0., 1.);

            // define texture coordinates
            let uv = Vec2::new(
                x_node as f32 / num_x_nodes as f32,
                y_node as f32 / num_y_nodes as f32,
            );

            positions.push(p);
            normals.push(n);
            texture.push(uv);

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

pub fn change_material(
    global_state: ResMut<GlobalState>,
    mut mesh_query: Query<&mut Handle<Mesh>>,
    mut assets: ResMut<Assets<Mesh>>,
    mut terrain_material: ResMut<Assets<TerrainMaterial>>,
) {
    if let Ok(handle) = mesh_query.get_single_mut() {
        let mut mesh = assets.get_mut(&handle);
        if mesh.is_some() {
            let positions = mesh.unwrap().attribute(Mesh::ATTRIBUTE_POSITION).unwrap();
            let normals = mesh.unwrap().attribute(Mesh::ATTRIBUTE_NORMAL).unwrap();

            for vertex in positions {}

            // let mut new_positions = vec![];
            // let mut new_normals = vec![];

            // let y_north = positions[1] + DELTA;
            // let y_south = positions.y - DELTA;

            // let x_west = positions.x - DELTA;
            // let x_east = positions.x + DELTA;

            // // X & Y positions for noise
            // let p = vec2<f32>(vertex.position.x, vertex.position.y);
            // let p_west = vec2<f32>(x_west, vertex.position.y);
            // let p_east = vec2<f32>(x_east, vertex.position.y);
            // let p_north = vec2<f32>(vertex.position.x, y_north);
            // let p_south = vec2<f32>(vertex.position.x, y_south);

            // // // Z height from noise
            // let z = z_height(p, params);
            // let z_west = z_height(p_west, params);
            // let z_east = z_height(p_east, params);
            // let z_north = z_height(p_north, params);
            // let z_south = z_height(p_south, params);

            //     mesh.unwrap().insert_attribute(Mesh::ATTRIBUTE_POSITION, temporary);
        }
    }
}
