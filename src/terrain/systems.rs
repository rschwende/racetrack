use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::components::*;
use crate::terrain::components::*;

pub const Y_SUB_MAX_LEN: f32 = 0.1;
pub const X_SUB_MAX_LEN: f32 = 0.1;

pub fn spawn_terrain(
    global_resource: ResMut<GlobalResource>,
    track_resource: Res<TrackResource>,
    mut commands: Commands,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut terrain_material_asset: ResMut<Assets<TerrainMaterial>>,
    mut material_asset: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    // create terrain material
    let terrain = TerrainMaterial {
        material_params: MaterialParams {
            base_color: Color::rgb(1.0, 0.0, 0.0),
        },
        noise_params: NoiseParams::new(&global_resource),
        track_image: track_resource.track_map_image_handle.clone(),
        track_texture: track_resource.track_texture_handle.clone(),
    };

    // create mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    terrain_mesh(&global_resource, &mut mesh);

    // spawn mesh
    commands.spawn((
        MaterialMeshBundle {
            mesh: mesh_assets.add(mesh),
            material: terrain_material_asset.add(terrain),
            visibility: Visibility::Visible,
            ..default()
        },
        TerrainElement,
    ));

    // test cube
    commands.spawn(PbrBundle {
        mesh: mesh_assets.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: material_asset.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });
}

pub fn terrain_mesh(global_resource: &ResMut<GlobalResource>, mesh: &mut Mesh) -> bool {
    // vectors that define mesh
    let mut indices = vec![];
    let mut positions = vec![];
    let mut normals = vec![];
    let mut texture = vec![];

    // mesh density
    let num_y_nodes =
        ((global_resource.y_max - global_resource.y_min) / Y_SUB_MAX_LEN).ceil() as u32;
    let num_x_nodes =
        ((global_resource.x_max - global_resource.x_min) / X_SUB_MAX_LEN).ceil() as u32;

    // define vertices
    for y_node in 0..num_y_nodes {
        let y = y_node as f32 / num_y_nodes as f32
            * (global_resource.y_max - global_resource.y_min)
            + global_resource.y_min;

        for x_node in 0..num_x_nodes {
            let x = x_node as f32 / num_x_nodes as f32
                * (global_resource.x_max - global_resource.x_min)
                + global_resource.x_min;

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

// update noise parameters of Terrain Material to global state
pub fn update_noise_params(
    global_resource: ResMut<GlobalResource>,
    material_query: Query<(Entity, &Handle<TerrainMaterial>)>,
    mut terrain_material_asset: ResMut<Assets<TerrainMaterial>>,
) {
    let (_id, mat_handle) = material_query.get_single().unwrap();
    let material = terrain_material_asset.get_mut(mat_handle).unwrap();

    material.noise_params = NoiseParams::new(&global_resource);
}
