use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}};
//use std::f32::consts::PI;

use crate::track::components::*;

pub const RAD_SUB:f32 = 0.1;    // max subdivision radial delta (ft)
pub const ARC_SUB:f32 = 0.1;    // max subdivision arc length (ft)


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
    let mesh = corner_mesh(2., 180., 2.);

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
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

    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
    //     ..Default::default()
    // });

}

pub fn corner_mesh(inner_radius: f32, curve_angle: f32, track_width: f32) -> Mesh {

    let mut indices = vec![];
    let mut positions = vec![]; 
    let mut normals = vec![];
    let mut texture = vec![];
    
    let outer_radius = inner_radius + track_width;
    let max_angle = (ARC_SUB/outer_radius).asin().to_degrees();                                 // max angle at curve center to maintain "ARC_SUB" at outermost vertices
    //let num_arc_sub: u32 = (((curve_angle % max_angle) + max_angle) % max_angle + 1.) as u32;        // number of arc subdivisions required given "curve_angle" and "ARC_SUB"

    //let num_rad_sub: u32 = (((track_width % RAD_SUB) + RAD_SUB) % RAD_SUB + 1.) as u32;              // number of radial subdivisions

    let num_arc_sub = 10;
    let num_rad_sub = 10;
    
    for i in 0..num_rad_sub {
        
        let rad_offset = i as f32 / num_rad_sub as f32 * track_width;
        let h = inner_radius + rad_offset;

        for j in 0..num_arc_sub {

            let arc_offset = j as f32 / num_arc_sub as f32 * curve_angle;

            // define vertices
            let new_point = Point {

                // define position
                p: Vec3::new(
                    h * arc_offset.to_radians().cos(),
                    h * arc_offset.to_radians().sin(),
                    0. 
                ),

                // define normals
                n: Vec3::new(0., 1., 0.),

                // define uv
                uv: Vec2::new(
                    1. - ((j / num_arc_sub) as f32),
                    (i / num_rad_sub) as f32
                )

            };

            positions.push(new_point.p);
            normals.push(new_point.n);
            texture.push(new_point.uv);

            // define indices
            if i < (num_rad_sub - 1) && j < (num_arc_sub - 1) {
                
                // first triangle
                    indices.push((i + 1) * num_arc_sub + j);
                    indices.push((i + 1) * num_arc_sub + j + 1 - num_arc_sub);
                    indices.push((i + 1) * num_arc_sub + j - num_arc_sub);

                // second triangle
                    indices.push((i + 1) * num_arc_sub + j);
                    indices.push((i + 1) * num_arc_sub + j + 1);
                    indices.push((i + 1) * num_arc_sub + j + 1 - num_arc_sub);
            }

        }
    }

    // define mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, texture);
    mesh //return

}



