// site this: https://github.com/clynamen/bevy_terrain/blob/0.0.1/src/gizmo.rs

use std::f32::consts::PI;

use bevy::prelude::*;

pub const AXIS_SIZE: f32 = 5.;

pub fn spawn_axis (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    //transform: Transform
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: AXIS_SIZE / 5. })),
        material: materials.add(Color::rgb(0.0, 0.0, 0.0).into()),
        ..Default::default()
        });
    
    // x red
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder { radius: AXIS_SIZE / 20., height: AXIS_SIZE, ..default() })),
        material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
        transform: Transform::from_rotation(Quat::from_rotation_z(PI/2.)).with_translation(Vec3::new(AXIS_SIZE / 2., 0.0, 0.0)),
        ..Default::default()
        });

    // y green
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder { radius: AXIS_SIZE / 20., height: AXIS_SIZE, ..default() })),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        transform: Transform::from_translation(Vec3::new(0.0, 2.5, 0.0)),
        ..Default::default()
    });

    // z blue
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cylinder { radius: AXIS_SIZE / 20., height: AXIS_SIZE, ..default() })),
        material: materials.add(Color::rgb(0.0, 0.0, 1.0).into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(PI/2.)).with_translation(Vec3::new(0.0, 0.0, AXIS_SIZE / 2.)),
        ..Default::default()
    });

}