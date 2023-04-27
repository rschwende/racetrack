// adapted from bevy_egui example: https://github.com/mvlabat/bevy_egui/blob/v0.20.1/examples/side_panel.rs

use bevy::{prelude::*, render::camera::Projection, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};

use crate::{GlobalState, ui::components::*};

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

pub fn configure_ui_state_system(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
}

pub fn ui_example_system(
    mut global_state: ResMut <GlobalState>,
    mut ui_state: ResMut<UiState>,
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::right("right_panel")
        .default_width(400.0)
        .show(ctx, |ui| {
            ui.heading("Terrain Parameters");

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));

            ui.add(egui::Slider::new(&mut global_state.frequency_scale, 0.0..=100.0).text("Frequency Scale"));
            ui.add(egui::Slider::new(&mut global_state.amplitude_scale, 0.0..=100.0).text("Amplitude Scale"));
            ui.add(egui::Slider::new(&mut global_state.octaves, 0..=100).text("Octaves"));
            ui.add(egui::Slider::new(&mut global_state.lacunarity, 0.0..=100.0).text("Lacunarity"));
            ui.add(egui::Slider::new(&mut global_state.gain, 0.0..=100.0).text("Gain"));





        });



    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
}

// pub fn setup_system(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
// ) {
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Plane {
//             size: 5.0,
//             subdivisions: 0,
//         })),
//         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//         ..Default::default()
//     });
//     commands.spawn(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//         transform: Transform::from_xyz(0.0, 0.5, 0.0),
//         ..Default::default()
//     });
//     commands.spawn(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             shadows_enabled: true,
//             ..Default::default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..Default::default()
//     });

//     let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
//     let camera_transform =
//         Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
//     commands.insert_resource(OriginalCameraTransform(camera_transform));

//     commands.spawn(Camera3dBundle {
//         transform: camera_transform,
//         ..Default::default()
//     });
// }

// pub fn update_camera_transform_system(
//     occupied_screen_space: Res<OccupiedScreenSpace>,
//     original_camera_transform: Res<OriginalCameraTransform>,
//     windows: Query<&Window, With<PrimaryWindow>>,
//     mut camera_query: Query<(&Projection, &mut Transform)>,
// ) {
//     let (camera_projection, mut transform) = match camera_query.get_single_mut() {
//         Ok((Projection::Perspective(projection), transform)) => (projection, transform),
//         _ => unreachable!(),
//     };

//     let distance_to_target = (CAMERA_TARGET - original_camera_transform.translation).length();
//     let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
//     let frustum_width = frustum_height * camera_projection.aspect_ratio;

//     let window = windows.single();

//     let left_taken = occupied_screen_space.left / window.width();
//     let right_taken = occupied_screen_space.right / window.width();
//     let top_taken = occupied_screen_space.top / window.height();
//     let bottom_taken = occupied_screen_space.bottom / window.height();
//     transform.translation = original_camera_transform.translation
//         + transform.rotation.mul_vec3(Vec3::new(
//             (right_taken - left_taken) * frustum_width * 0.5,
//             (top_taken - bottom_taken) * frustum_height * 0.5,
//             0.0,
//         ));
// }