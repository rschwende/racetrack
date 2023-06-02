use bevy::{
    math::Vec4Swizzles,
    prelude::*,
    render::{color::Color, mesh::Indices, render_resource::PrimitiveTopology},
};

use crate::{
    components::*,
    systems::{MAX_TRACK_HEIGHT, MIN_TRACK_HEIGHT},
};
use std::{convert::identity, f32::consts::PI};

// track params
pub const RAD_SUB_MAX_LEN: f32 = 0.3; // max subdivision radial delta (ft)
pub const ARC_SUB_MAX_LEN: f32 = 0.3; // max subdivision arc length (ft)
pub const MAX_CURVATURE: f32 = 3.;
pub const TERRAIN_OFFSET: f32 = 25.; // how much the terrain extends past track

// track to terrain blending parameters
pub const TRANSITION_WIDTH: f32 = 5.;
pub const TRANS_RAD_SUB_MAX_LEN: f32 = 1.;

// this will be replaced by a UI
pub fn create_track_list(
    mut track_resource: ResMut<TrackResource>,
    mut images: ResMut<Assets<Image>>,
) {
    // list track elements below

    // track element 1
    let track_element = TrackElement2D {
        curvature: 0.,
        curve_angle: 0.,
        start_bank_angle: 0.,
        end_bank_angle: 0.,
        width: 8.,
        length: 10.,
    };
    track_resource.track_list.push(track_element);

    // track element 2
    let track_element = TrackElement2D {
        curvature: 0.1,
        curve_angle: 90.,
        start_bank_angle: 0.,
        end_bank_angle: -30.,
        width: 8.,
        length: 20.,
    };
    track_resource.track_list.push(track_element);

    // track element 3
    let track_element = TrackElement2D {
        curvature: 0.1,
        curve_angle: 90.,
        start_bank_angle: -30.,
        end_bank_angle: 0.,
        width: 8.,
        length: 0.,
    };
    track_resource.track_list.push(track_element);

    // track element 4
    let track_element = TrackElement2D {
        curvature: 0.,
        curve_angle: 0.,
        start_bank_angle: 0.,
        end_bank_angle: 0.,
        width: 8.,
        length: 20.,
    };
    track_resource.track_list.push(track_element);

    // track element 5
    let track_element = TrackElement2D {
        curvature: 0.1,
        curve_angle: 90.,
        start_bank_angle: 0.,
        end_bank_angle: -30.,
        width: 8.,
        length: 20.,
    };
    track_resource.track_list.push(track_element);

    // track element 6
    let track_element = TrackElement2D {
        curvature: 0.1,
        curve_angle: 90.,
        start_bank_angle: -30.,
        end_bank_angle: 0.,
        width: 8.,
        length: 0.,
    };
    track_resource.track_list.push(track_element);

    // track element 7
    let track_element = TrackElement2D {
        curvature: 0.,
        curve_angle: 0.,
        start_bank_angle: 0.,
        end_bank_angle: 0.,
        width: 8.,
        length: 10.,
    };
    track_resource.track_list.push(track_element);
}

/// spawns track elements
pub fn spawn_track(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut global_resource: ResMut<GlobalResource>,
    track_resource: Res<TrackResource>,
    mut mesh_resource: ResMut<MeshResource>,
) {
    let mut prev_transform = Transform::IDENTITY;

    let track_material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(1.0, 1.0, 1.0),
        unlit: true,
        ..default()
    });

    // iterate through track list, spawn to scene, and update track extent
    for track_element in &track_resource.track_list {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let mut new_transform = Transform::IDENTITY;
        track_mesh_2d(&track_element, &mut mesh, &mut new_transform);

        let mesh_handle = meshes.add(mesh);

        // store values to be used in render to texture pass
        mesh_resource.track_mesh_list.push(mesh_handle.clone());
        mesh_resource.track_mesh_transform_list.push(prev_transform);

        commands.spawn((
            PbrBundle {
                mesh: mesh_handle.clone(),
                material: track_material_handle.clone(),
                transform: prev_transform,
                visibility: Visibility::Hidden,
                ..default()
            },
            TrackElement,
        ));

        prev_transform = prev_transform * new_transform;

        // update track extent (terrain plane size)
        let p: Vec3 = (prev_transform.compute_matrix() * Vec4::new(0., 0., 0., 1.)).xyz();

        if p.x + TERRAIN_OFFSET > global_resource.x_max {
            global_resource.x_max = p.x + TERRAIN_OFFSET;
        }

        if p.x - TERRAIN_OFFSET < global_resource.x_min {
            global_resource.x_min = p.x - TERRAIN_OFFSET;
        }

        if p.y + TERRAIN_OFFSET > global_resource.y_max {
            global_resource.y_max = p.y + TERRAIN_OFFSET;
        }

        if p.y - TERRAIN_OFFSET < global_resource.y_min {
            global_resource.y_min = p.y - TERRAIN_OFFSET;
        }
    }

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 20000.,
            shadows_enabled: false,
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 50.0),
        ..default()
    });
}

/// creates track element mesh in passed mesh and passed transform ends as start position of
/// next track element
pub fn track_mesh_2d(
    track: &TrackElement2D,
    track_mesh: &mut Mesh,
    arc_transform: &mut Transform,
) -> bool {
    // vectors that define mesh
    let mut indices = vec![];
    let mut positions = vec![];
    let mut normals = vec![];
    let mut colors = vec![];

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
    // number of transition nodes at edge of track width
    let num_trans_nodes = (TRANSITION_WIDTH / TRANS_RAD_SUB_MAX_LEN).ceil() as u32 + 1;

    let total_rad_nodes = num_rad_nodes + 2 * num_trans_nodes - 2;

    let total_width = track.width + 2. * TRANSITION_WIDTH;

    // initiate banking transform for radial vertices
    let mut bank_transform = Transform::IDENTITY;

    // define vertices
    for curr_arc_node in 0..num_arc_nodes {
        let length_ratio = curr_arc_node as f32 / (num_arc_nodes - 1) as f32;

        // arc transform
        create_arc_transform(arc_transform, track, length, length_ratio);
        let arc_matrix = arc_transform.compute_matrix();

        // banking transform
        create_bank_transform(&mut bank_transform, track, length_ratio);
        let bank_matrix = bank_transform.compute_matrix();

        let mut curr_width: f32;
        let mut vertex_color_red: f32; // represents track to terrain blend

        for curr_rad_node in 0..total_rad_nodes {
            // -Y transition zone
            if curr_rad_node < (num_trans_nodes - 1) {
                // define position
                curr_width = curr_rad_node as f32 / (num_trans_nodes - 1) as f32 * TRANSITION_WIDTH
                    - total_width / 2.;

                // define color
                vertex_color_red = 1. - curr_rad_node as f32 / (num_trans_nodes - 1) as f32;

            // +_Y transition zone
            } else if curr_rad_node > (num_trans_nodes + num_rad_nodes - 2) {
                // define position
                curr_width = (curr_rad_node - (num_trans_nodes + num_rad_nodes - 2)) as f32
                    / (num_trans_nodes - 1) as f32
                    * TRANSITION_WIDTH
                    + track.width
                    + TRANSITION_WIDTH
                    - total_width / 2.;

                // define color
                vertex_color_red = (curr_rad_node - (num_trans_nodes + num_rad_nodes - 2)) as f32
                    / (num_trans_nodes - 1) as f32;

            // track
            } else {
                // define position
                curr_width = ((curr_rad_node - (num_trans_nodes - 1)) as f32
                    / (num_rad_nodes - 1) as f32)
                    * track.width
                    + TRANSITION_WIDTH
                    - total_width / 2.;

                // define color
                vertex_color_red = 0.;
            }

            // define position
            let p = Vec4::new(0., -curr_width, 0., 1.);
            let p: Vec3 = (arc_matrix * bank_matrix * p).xyz();

            let vertex_color_green =
                (p.z - MIN_TRACK_HEIGHT) / (MAX_TRACK_HEIGHT - MIN_TRACK_HEIGHT);

            // define normals
            let n = Vec4::new(0., 0., 1., 1.);
            let n: Vec3 = (arc_matrix * bank_matrix * n).xyz();

            // define colors
            let c = Color::rgb(vertex_color_red, vertex_color_green, 0.);

            positions.push(p);
            normals.push(n);
            colors.push(c.as_rgba_f32());

            // define indices
            if curr_arc_node < (num_arc_nodes - 1) && curr_rad_node < (total_rad_nodes - 1) {
                // first triangle
                indices
                    .push((curr_arc_node + 1) * total_rad_nodes + curr_rad_node - total_rad_nodes);
                indices.push(
                    (curr_arc_node + 1) * total_rad_nodes + curr_rad_node + 1 - total_rad_nodes,
                );
                indices.push((curr_arc_node + 1) * total_rad_nodes + curr_rad_node + 1);

                // second triangle
                indices
                    .push((curr_arc_node + 1) * total_rad_nodes + curr_rad_node - total_rad_nodes);
                indices.push((curr_arc_node + 1) * total_rad_nodes + curr_rad_node + 1);
                indices.push((curr_arc_node + 1) * total_rad_nodes + curr_rad_node);
            }
        }
    }

    // define mesh
    track_mesh.set_indices(Some(Indices::U32(indices)));
    track_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    track_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    track_mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    true //return
}

/// creates transformation matrix for current length on arc length
fn create_arc_transform(
    transform: &mut Transform,
    track: &TrackElement2D,
    length: f32,
    length_ratio: f32,
) {
    let curr_length = length_ratio * length;

    if track.curvature == 0. {
        // straight
        let translation = Transform::from_xyz(curr_length, 0., 0.);
        *transform = Transform::IDENTITY * translation;
    } else {
        // curve
        let theta = (curr_length * track.curvature).abs();

        let x = theta.sin() / track.curvature.abs();
        let y = (1. - theta.cos()) / track.curvature.abs();
        let z = 0.;

        let translation: Transform;
        let rotation: Transform;

        if track.curvature > 0. {
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

fn create_bank_transform(transform: &mut Transform, track: &TrackElement2D, length_ratio: f32) {
    let curr_bank_angle = (track.start_bank_angle
        + smoothstep(length_ratio) * (track.end_bank_angle - track.start_bank_angle))
        * PI
        / 180.;

    let banking = Transform::from_rotation(Quat::from_rotation_x(curr_bank_angle));

    let height_offset = (track.width / 2. * curr_bank_angle.sin()).abs();
    let translation = Transform::from_translation(Vec3::new(0., 0., height_offset));

    *transform = translation * banking;
}

/// GLSL style smoothstep function with edges at 0 and 1
/// example: https://en.wikipedia.org/wiki/Smoothstep
fn smoothstep(value: f32) -> f32 {
    if value < 0.0 {
        return 0.0;
    } else if value > 1.0 {
        return 1.0;
    }

    return value * value * (3. - 2. * value);
}
