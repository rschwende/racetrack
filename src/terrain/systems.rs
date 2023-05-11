use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use noisy_bevy::fbm_simplex_2d_seeded;

use crate::terrain::components::*;
use crate::GlobalState;

pub const Y_SUB_MAX_LEN: f32 = 2.;
pub const X_SUB_MAX_LEN: f32 = 2.;
pub const DELTA: f32 = 0.1;

pub fn spawn_terrain(
    mut global_state: ResMut<GlobalState>,
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    //mut terrain_material: ResMut<Assets<TerrainMaterial>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let mut terrain_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    build_terrain_mesh(&global_state, &mut terrain_mesh);

    let material_mesh = MaterialMeshBundle {
        mesh: mesh_assets.add(terrain_mesh),
        material: material_assets.add(StandardMaterial {
            base_color: Color::hex("#ffd891").unwrap(),
            metallic: 0.0,
            perceptual_roughness: 0.5,
            ..default()
        }),
        ..default()
    };

    // setup terrain bundle for identification by query
    commands.spawn(TerrainBundle {
        marker: Terrain,
        matmesh: material_mesh,
    });

    // // create the mesh
    // terrain_mesh(&global_state, &mut new_terrain.);

    // // let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    // // terrain_mesh(&global_state, &mut mesh);

    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(new_terrain.mesh),
    //     material: terrain_material.add(new_terrain.material),
    //     ..default()
    // });

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

    //test cube
    commands.spawn(PbrBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: material_assets.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
    });
}

pub fn build_terrain_mesh(global_state: &ResMut<GlobalState>, mesh: &mut Mesh) -> bool {
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

fn z_height(p: Vec2, global_state: &ResMut<GlobalState>) -> f32 {
    let z = fbm_simplex_2d_seeded(
        p * global_state.frequency_scale,
        global_state.octaves,
        global_state.lacunarity,
        global_state.gain,
        1.,
    ) * global_state.amplitude_scale;
    return z;
}

pub fn change_material(
    global_state: ResMut<GlobalState>,
    mut mesh_query: Query<(Entity, &Handle<Mesh>, &Handle<StandardMaterial>), With<Terrain>>,
    mut mesh_asset: ResMut<Assets<Mesh>>,
    //mut mat_asset: Assets<StandardMaterial>,
    //mut terrain_material: ResMut<Assets<TerrainMaterial>>,
) {
    //println!("here1");

    let empty = mesh_query.iter().count();

    //println!("{}", empty);

    let (_id, mesh_handle, mat_handle) = mesh_query.get_single().unwrap();

    let mesh = mesh_asset.get_mut(mesh_handle).unwrap();
    //let material = mat_asset.get(mat_handle).unwrap();

    // if let Ok(handle) = mesh_query.get_single() {
    //     let mut mesh = assets.get_mut(handle).unwrap();
    //     println!("here2");
    // }

    // println!("here2");

    let positions = mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap().clone();
    //let normals = mesh.attribute(Mesh::ATTRIBUTE_NORMAL).unwrap();

    let positions_vec = positions.as_float3().unwrap();
    //let normals_vec = normals.as_float3().unwrap();

    let mut new_positions = vec![];
    let mut new_normals = vec![];

    for vertex in positions_vec {
        let y_north = vertex[1] + DELTA;
        let y_south = vertex[1] - DELTA;

        let x_west = vertex[0] - DELTA;
        let x_east = vertex[0] + DELTA;

        // X & Y positions for noise
        let p = Vec2::new(vertex[0], vertex[1]);
        let p_west = Vec2::new(x_west, vertex[1]);
        let p_east = Vec2::new(x_east, vertex[1]);
        let p_north = Vec2::new(vertex[0], y_north);
        let p_south = Vec2::new(vertex[0], y_south);

        // println!("p: {:?}", p);
        // println!("p_north: {:?}", p_north);
        // println!("p_south: {:?}", p_south);
        // println!("p_east: {:?}", p_east);
        // println!("p_west: {:?}", p_west);

        // Z height from noise
        let z = z_height(p, &global_state);
        let z_west = z_height(p_west, &global_state);
        let z_east = z_height(p_east, &global_state);
        let z_north = z_height(p_north, &global_state);
        let z_south = z_height(p_south, &global_state);

        // println!("z: {:?}", z);
        // println!("z_north: {:?}", z_north);
        // println!("z_south: {:?}", z_south);
        // println!("z_east: {:?}", z_east);
        // println!("z_west: {:?}", z_west);

        // define position
        new_positions.push(Vec3::new(vertex[0], vertex[1], z));

        // define normals
        let stangent = Vec3::new(2. * DELTA, 0., z_east - z_west);
        let ttangent = Vec3::new(0., 2. * DELTA, z_north - z_south);

        new_normals.push((stangent.cross(ttangent)).normalize());

        // println!("stangent: {:?}", stangent);
        // println!("ttangent: {:?}", ttangent);
        // println!("cross: {:?}", stangent.cross(ttangent));
        // println!("normalize: {:?}", (stangent.cross(ttangent)).normalize());
    }

    //println!("{:?}", new_normals);

    // remove current attributes
    mesh.remove_attribute(Mesh::ATTRIBUTE_POSITION);
    mesh.remove_attribute(Mesh::ATTRIBUTE_NORMAL);

    // add new attributes
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, new_positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, new_normals);
}
