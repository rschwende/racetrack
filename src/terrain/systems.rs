use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::terrain::components::*;
use crate::GlobalState;

pub const Y_SUB_MAX_LEN: f32 = 0.05;
pub const X_SUB_MAX_LEN: f32 = 0.05;

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
    mut material_query: Query<&mut Handle<TerrainMaterial>>,
    mut terrain_material: ResMut<Assets<TerrainMaterial>>,
) {
    let mut material = material_query
        .get_single_mut()
        .expect("Error: Could not find a single Terrain Material.");

    let new_terrain = TerrainMaterial {
        params: TerrainMaterialParams::new(&global_state),
    };

    *material = terrain_material.add(new_terrain);
}

//    if let Ok(mut material) = material_query.get_single_mut()
