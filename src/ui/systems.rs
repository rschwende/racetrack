// adapted from bevy_egui example: https://github.com/mvlabat/bevy_egui/blob/v0.20.1/examples/side_panel.rs

use bevy::{prelude::*, render::camera::Projection, window::PrimaryWindow};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSettings};

use crate::{ui::components::*, GlobalResource};

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

pub fn configure_ui_state_system(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
}

pub fn ui_example_system(
    mut global_resource: ResMut<GlobalResource>,
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

            ui.add(
                egui::Slider::new(&mut global_resource.frequency_scale, 0.0..=0.2)
                    .text("Frequency Scale"),
            );
            ui.add(
                egui::Slider::new(&mut global_resource.amplitude_scale, 0.0..=10.0)
                    .text("Amplitude Scale"),
            );
            ui.add(egui::Slider::new(&mut global_resource.octaves, 0..=4).text("Octaves"));
            ui.add(
                egui::Slider::new(&mut global_resource.lacunarity, 0.0..=1.0).text("Lacunarity"),
            );
            ui.add(egui::Slider::new(&mut global_resource.gain, 0.0..=1.0).text("Gain"));

            let print_button = ui.button("Print Variables").clicked();

            if print_button {
                print_vars(global_resource);
            }
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

fn print_vars(global_resource: ResMut<GlobalResource>) {
    println!("frequency_scale: {}", global_resource.frequency_scale);
    println!("amplitude_scale: {}", global_resource.amplitude_scale);
    println!("octaves: {}", global_resource.octaves);
    println!("lacunarity: {}", global_resource.lacunarity);
    println!("gain: {}", global_resource.gain);
}
