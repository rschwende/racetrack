use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}, math::Vec4Swizzles};
//use std::f32::consts::PI;

use crate::track::components::*;

pub const RAD_SUB_MAX_LEN:f32 = 0.3;    // max subdivision radial delta (ft)
pub const ARC_SUB_MAX_LEN:f32 = 0.3;    // max subdivision arc length (ft)
pub const MAX_CURVATURE: f32 = 3.;
pub const PI: f32 = 3.14159265358979323846264338327950288f32;


// pub fn spawn_test_scene(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     // spawn a cube and a light
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//         transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
//         ..Default::default()
//     });
//     commands.spawn(PointLightBundle {
//         transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
//         ..Default::default()
//     });

// }


// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

//     // Positions of the vertices
//     // See https://bevy-cheatbook.github.io/features/coords.html
//     mesh.insert_attribute(
//         Mesh::ATTRIBUTE_POSITION,
//         vec![[0., 0., 0.], [1., 2., 1.], [2., 0., 0.]],
//     );

//     // In this example, normals and UVs don't matter,
//     // so we just use the same value for all of them
//     mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
//     mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);

//     // A triangle using vertices 0, 2, and 1.
//     // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
//     mesh.set_indices(Some(mesh::Indices::U32(vec![0, 2, 1])));

//     commands.spawn(PbrBundle {
//         mesh: meshes.add(mesh),
//         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//         ..default()
//     });

//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             shadows_enabled: true,
//             ..default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..default()
//     });

//     commands.spawn(Camera3dBundle {
//         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..default()
//     });
// }



// pub fn spawn_CSYS(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {

//     let rotations = [
//         Quat::from_rotation_x(0.0),
//         Quat::from_rotation_x(-PI / 2.),
//         Quat::from_rotation_z(PI / 2.),
//     ];


//     for rotation in rotations.into_iter() {

//         commands.spawn(PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Cylinder { radius: 0.05, height: 5.0, ..default() })),
//             material: materials.add(Color::rgb(1.0, 1.0, 1.0).into()),
//             transform: Transform::from_xyz(0., 0., 0.).with_rotation(rotation),
//             ..default()
//         });
//     }


// }

//-----------------------------------------------------------------------------------------------------

pub fn spawn_track_element(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {

    // 0 
    let transform0 = Transform::IDENTITY;
    
    // 1
    let mut mesh1 = Mesh::new(PrimitiveTopology::TriangleList);
    let mut transform1 = Transform::IDENTITY;
    let track1 = TrackElement2D{
        curvature: 0.05,
        curve_angle: 90.,
        width: 5.,
        length: 10.,
    };

    let result = track_mesh_2d(track1, &mut mesh1, &mut transform1);

    // 2
    let mut mesh2 = Mesh::new(PrimitiveTopology::TriangleList);
    let mut transform2 = Transform::IDENTITY;
    let track2 = TrackElement2D{
        curvature: 0.,
        curve_angle: 180.,
        width: 5.,
        length: 30.,
    };

    let result = track_mesh_2d(track2, &mut mesh2, &mut transform2);

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh1),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: transform0,
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh2),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        transform: transform1,
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 8.0, 0.0),
        ..default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
        ..Default::default()
    });

}




/// 
/// 
pub fn track_mesh_2d(track: TrackElement2D, track_mesh: &mut Mesh, transform: &mut Transform) -> bool {
    
    // vectors that define mesh
    let mut indices = vec![];
    let mut positions = vec![]; 
    let mut normals = vec![];
    //let mut texture = vec![];

    // check inner radius > min
    if track.curvature > MAX_CURVATURE {return false;}    // curve is below min radius

    // determine length
    let length: f32;
    if track.curvature == 0. {
        // straight
        length = track.length;
    } else {
        // curve
        length = (2.* PI * track.curve_angle / 360. / track.curvature).abs(); 
    }
    
    // mesh density
    //let num_arc_nodes = (length / ARC_SUB_MAX_LEN).ceil() as u32;
    //let num_rad_nodes = (track.width / RAD_SUB_MAX_LEN).ceil() as u32;

    let num_arc_nodes = 10;
    let num_rad_nodes = 6;

    //
    for curr_arc_node in 0..num_arc_nodes {

        let curr_length = curr_arc_node as f32 / num_arc_nodes as f32 * length;

        arc_transform(transform, curr_length, track.curvature);
        let matrix = transform.compute_matrix();

        for curr_rad_node in 0..num_rad_nodes {

            let curr_width = curr_rad_node as f32 / num_rad_nodes as f32 * track.width - track.width / 2.;
            
            // define position
            let p = Vec4::new(0., - curr_width, 0., 1.);
            let p: Vec3 = (matrix * p).xyz();

            // define normals
            let n = Vec4:: new (0., 0., 1., 0.);
            let n: Vec3 = (matrix * n).xyz();

            positions.push(p);
            normals.push(n);

            // define indices
            if curr_arc_node < (num_arc_nodes - 1) && curr_rad_node < (num_rad_nodes - 1) {

                // first triangle
                indices.push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node - num_rad_nodes);
                indices.push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node + 1 - num_rad_nodes);
                indices.push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node + 1);

                // second triangle
                indices.push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node - num_rad_nodes);
                indices.push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node + 1);
                indices.push((curr_arc_node + 1) * num_rad_nodes + curr_rad_node);

            }

        }

    }
    
    // define mesh
    //track_mesh = &Mesh::new(PrimitiveTopology::TriangleList);
    track_mesh.set_indices(Some(Indices::U32(indices)));
    track_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    track_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    //mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, texture);
    true //return

}



fn arc_transform(transform: &mut Transform, curr_length: f32, curvature: f32) {


    if curvature == 0. {
        // straight
        let translation = Transform::from_xyz(curr_length, 0., 0.);
        *transform = Transform::IDENTITY * translation;

    } else {
        // curve
        let theta = (curr_length * curvature).abs();

        let x = theta.sin() / curvature;
        let y = (1. - theta.cos()) / curvature;
        let z = 0.;

        let translation: Transform;
        let rotation: Transform;

        if curvature > 0. {
            // left curve
            translation = Transform::from_xyz(x, -y, z);
            rotation = Transform::from_rotation(Quat::from_rotation_z(-theta));
        } else {
            // right curve
            translation = Transform::from_xyz(x, y, z);
            rotation = Transform::from_rotation(Quat::from_rotation_z(theta));
        }

        *transform = Transform::IDENTITY * translation * rotation;
    }

}
