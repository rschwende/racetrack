use bevy::{
    math::Vec4Swizzles,
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::track::components::*;
use crate::GlobalState;
use std::f32::consts::PI;

pub const RAD_SUB_MAX_LEN: f32 = 0.3; // max subdivision radial delta (ft)
pub const ARC_SUB_MAX_LEN: f32 = 0.3; // max subdivision arc length (ft)
pub const MAX_CURVATURE: f32 = 3.;
pub const TERRAIN_OFFSET: f32 = 50.; // how much the terrain extends past track

/// spawns a list of track elements
pub fn spawn_track_element(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut global_state: ResMut<GlobalState>,
) {
    // list of track elements
    let track_list = vec![
        // track element 1
        TrackElement2D {
            curvature: 0.1,
            curve_angle: 90.,
            width: 8.,
            length: 10.,
        },
        // track element 2
        TrackElement2D {
            curvature: 0.,
            curve_angle: 90.,
            width: 8.,
            length: 20.,
        },
        // track element 3
        TrackElement2D {
            curvature: 0.1,
            curve_angle: 180.,
            width: 8.,
            length: 10.,
        },
        // track element 4
        TrackElement2D {
            curvature: 0.,
            curve_angle: 90.,
            width: 8.,
            length: 20.,
        },
        // track element 5
        TrackElement2D {
            curvature: 0.1,
            curve_angle: 90.,
            width: 8.,
            length: 20.,
        },
    ];

    let mut prev_transform = Transform::IDENTITY;

    // iterate through track list, spawn to scene, and update track extent
    for track in track_list {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut new_transform = Transform::IDENTITY;
        track_mesh_2d(track, &mut mesh, &mut new_transform);

        commands.spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            transform: prev_transform,
            ..default()
        });

        prev_transform = prev_transform * new_transform;

        // update track extent
        let p: Vec3 = (prev_transform.compute_matrix() * Vec4::new(0., 0., 0., 1.)).xyz();

        if p.x + TERRAIN_OFFSET > global_state.x_max {
            global_state.x_max = p.x + TERRAIN_OFFSET;
        }

        if p.x - TERRAIN_OFFSET < global_state.x_min {
            global_state.x_min = p.x - TERRAIN_OFFSET;
        }

        if p.y + TERRAIN_OFFSET > global_state.y_max {
            global_state.y_max = p.y + TERRAIN_OFFSET;
        }

        if p.y - TERRAIN_OFFSET < global_state.y_min {
            global_state.y_min = p.y - TERRAIN_OFFSET;
        }
    }

    // lighting
    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 40.0),
            ..default()
        },
        // // The default cascade config is designed to handle large scenes.
        // // As this example has a much smaller world, we can tighten the shadow
        // // bounds for better visual quality.
        // cascade_shadow_config: CascadeShadowConfigBuilder {
        //     first_cascade_far_bound: 4.0,
        //     maximum_distance: 10.0,
        //     ..default()
        // }
        // .into(),
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

/// creates track element mesh in passed mesh and passed transform ends as start position of
/// next track element
pub fn track_mesh_2d(
    track: TrackElement2D,
    track_mesh: &mut Mesh,
    transform: &mut Transform,
) -> bool {
    // vectors that define mesh
    let mut indices = vec![];
    let mut positions = vec![];
    let mut normals = vec![];
    //let mut texture = vec![];

    // check inner radius > min
    if track.curvature > MAX_CURVATURE {
        return false;
    } // curve is below min radius

    // determine length
    let length: f32;
    if track.curvature == 0. {
        // straight
        length = track.length;
    } else {
        // curve
        length = (2. * PI * track.curve_angle / 360. / track.curvature).abs();
    }

    // mesh density
    let num_arc_nodes = (length / ARC_SUB_MAX_LEN).ceil() as u32;
    let num_rad_nodes = (track.width / RAD_SUB_MAX_LEN).ceil() as u32;

    // define vertices
    for curr_arc_node in 0..num_arc_nodes {
        let curr_length = curr_arc_node as f32 / (num_arc_nodes - 1) as f32 * length;

        arc_transform(transform, curr_length, track.curvature);
        let matrix = transform.compute_matrix();

        for curr_rad_node in 0..num_rad_nodes {
            let curr_width =
                curr_rad_node as f32 / num_rad_nodes as f32 * track.width - track.width / 2.;

            // define position
            let p = Vec4::new(0., -curr_width, 0., 1.);
            let p: Vec3 = (matrix * p).xyz();

            // define normals
            let n = Vec4::new(0., 0., 1., 1.);
            //let n: Vec3 = (matrix * n).xyz();
            let n: Vec3 = n.xyz();

            positions.push(p);
            normals.push(n);

            // define indices
            if curr_arc_node < (num_arc_nodes - 1) && curr_rad_node < (num_rad_nodes - 1) {
                // first triangle
                indices.push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node - num_rad_nodes);
                indices
                    .push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node + 1 - num_rad_nodes);
                indices.push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node + 1);

                // second triangle
                indices.push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node - num_rad_nodes);
                indices.push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node + 1);
                indices.push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node);
            }
        }
    }

    // define mesh
    track_mesh.set_indices(Some(Indices::U32(indices)));
    track_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    track_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    //mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, texture);
    true //return
}

/// creates transformation matrix for passed length along track element
fn arc_transform(transform: &mut Transform, curr_length: f32, curvature: f32) {
    if curvature == 0. {
        // straight
        let translation = Transform::from_xyz(curr_length, 0., 0.);
        *transform = Transform::IDENTITY * translation;
    } else {
        // curve
        let theta = (curr_length * curvature).abs();

        let x = theta.sin() / curvature.abs();
        let y = (1. - theta.cos()) / curvature.abs();
        let z = 0.;

        let translation: Transform;
        let rotation: Transform;

        if curvature > 0. {
            // left curve
            translation = Transform::from_xyz(x, y, z);
            rotation = Transform::from_rotation(Quat::from_rotation_z(theta));
        } else {
            // right curve
            translation = Transform::from_xyz(x, -y, z);
            rotation = Transform::from_rotation(Quat::from_rotation_z(-theta));
        }

        *transform = Transform::IDENTITY * translation * rotation;
    }
}
