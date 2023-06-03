use std::slice::Windows;

use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy::render::camera::{Projection, RenderTarget, ScalingMode};
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::view::RenderLayers;
use bevy::window::PrimaryWindow;

use crate::components::*;
use crate::track_height_map::components::*;

pub fn create_height_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut images: ResMut<Assets<Image>>,
    mut global_resource: ResMut<GlobalResource>,
    mut track_resource: ResMut<TrackResource>,
    mut mesh_resource: ResMut<MeshResource>,
    mut texture_material_asset: ResMut<Assets<TextureMaterial>>,
) {
    // position
    let quad_size = Vec2::new(
        global_resource.x_max - global_resource.x_min,
        global_resource.y_max - global_resource.y_min,
    );

    let quad_center = Vec2::new(
        quad_size.x / 2. + global_resource.x_min,
        quad_size.y / 2. + global_resource.y_min,
    );

    // texture size
    let size = Extent3d {
        width: 2048,
        height: 2048,
        ..default()
    };

    // texture image
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba32Float,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);
    let image_handle = images.add(image);
    let first_pass_layer = RenderLayers::layer(1);

    // camera for render to texture pass
    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: ClearColorConfig::Custom(Color::WHITE),
                ..default()
            },
            camera: Camera {
                // render before the "main pass" camera
                order: -1,
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(quad_center.x, quad_center.y, 10.0))
                .looking_at(Vec3::new(quad_center.x, quad_center.y, 0.0), Vec3::Y),
            projection: OrthographicProjection {
                near: -15.0,
                scaling_mode: ScalingMode::Fixed {
                    width: quad_size.x,
                    height: quad_size.y,
                },
                ..default()
            }
            .into(),
            ..default()
        },
        first_pass_layer,
    ));

    let texture_material = TextureMaterial {};

    for (mesh_handle, transform) in mesh_resource
        .track_mesh_list
        .iter()
        .zip(mesh_resource.track_mesh_transform_list.iter())
    {
        commands.spawn((
            MaterialMeshBundle {
                mesh: mesh_handle.clone(),
                material: texture_material_asset.add(texture_material.clone()),
                transform: *transform,
                ..default()
            },
            RenderToTexturePass,
            first_pass_layer,
        ));
    }

    // plane for render to texture pass
    let plane_handle = meshes.add(Mesh::from(shape::Quad {
        size: quad_size,
        ..default()
    }));

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(image_handle.clone()),
        unlit: true,
        ..default()
    });

    // Main pass plane, with material containing the rendered first pass texture.
    commands.spawn((
        PbrBundle {
            mesh: plane_handle,
            material: material_handle,
            transform: Transform::from_xyz(quad_center.x, quad_center.y, 0.0),
            ..default()
        },
        PlaneElement,
    ));

    track_resource.track_map_image_handle = image_handle.clone();
}
