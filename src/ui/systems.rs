// adapted from bevy_egui example: https://github.com/mvlabat/bevy_egui/blob/v0.20.1/examples/side_panel.rs

use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Ui},
    EguiContexts,
};

use crate::{
    components::*,
    systems::despawn,
    terrain::components::TerrainMaterial,
    terrain::systems::spawn_terrain,
    track::systems::spawn_track,
    track_height_map::{components::TextureMaterial, systems::create_height_map},
    ui::components::*,
    GlobalResource,
};

/// bevy setup system
pub fn setup(mut ui_resource: ResMut<UIResource>, mut track_resource: ResMut<TrackResource>) {
    set_ui_resource(&mut ui_resource, &mut track_resource);
}

pub fn set_ui_resource(
    ui_resource: &mut ResMut<UIResource>,
    track_resource: &mut ResMut<TrackResource>,
) {
    ui_resource.new_index = track_resource.track_list.len() - 1;

    // default values for new track element
    ui_resource.new_track_element.curvature = 0.;
    ui_resource.new_track_element.curve_angle = 0.;
    ui_resource.new_track_element.start_bank_angle = 0.;
    ui_resource.new_track_element.end_bank_angle = 0.;
    ui_resource.new_track_element.width = 8.;
    ui_resource.new_track_element.length = 10.;
}

pub fn ui_system(mut global_resource: ResMut<GlobalResource>, mut contexts: EguiContexts) {
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
            ui.add(egui::Slider::new(&mut global_resource.scale, 0.1..=100.).text("Scale"));
            ui.add(
                egui::Slider::new(&mut global_resource.track_texture_scale, 0.5..=5.)
                    .text("Track Texture Scale"),
            );
            ui.add(
                egui::Slider::new(&mut global_resource.terrain_texture_scale, 0.5..=5.)
                    .text("Terrain Texture Scale"),
            );

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));

            ui.heading("View Options");

            ui.allocate_space(egui::Vec2::new(1.0, 10.0));

            ui.add(egui::Checkbox::new(
                &mut global_resource.show_track,
                "Show Track",
            ));
            ui.add(egui::Checkbox::new(
                &mut global_resource.show_track_texture,
                "Show Track Texture",
            ));
            ui.add(egui::Checkbox::new(
                &mut global_resource.show_terrain,
                "Show Terrain",
            ));
            ui.add(egui::Checkbox::new(
                &mut global_resource.show_track_map,
                "Show Track Map on Terrain",
            ));
        });
}

pub fn track_list_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut global_resource: ResMut<GlobalResource>,
    mut track_resource: ResMut<TrackResource>,
    mut mesh_resource: ResMut<MeshResource>,
    mut ui_resource: ResMut<UIResource>,
    mut contexts: EguiContexts,
    mut entity_query: Query<Entity, With<MyEntity>>,
    mut images: ResMut<Assets<Image>>,
    mut texture_material_asset: ResMut<Assets<TextureMaterial>>,
    mut terrain_material_asset: ResMut<Assets<TerrainMaterial>>,
) {
    let ctx = contexts.ctx_mut();

    egui::SidePanel::left("left_panel")
        .default_width(800.0)
        .show(ctx, |ui| {
            // updated track
            if ui
                .add(egui::Button::new("  Update Track").min_size(egui::Vec2::new(100., 50.)))
                .clicked()
            {
                despawn(&mut commands, &mut entity_query);

                spawn_track(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut global_resource,
                    &mut track_resource,
                    &mut mesh_resource,
                );

                create_height_map(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    &mut images,
                    &mut global_resource,
                    &mut track_resource,
                    &mut mesh_resource,
                    &mut texture_material_asset,
                );

                spawn_terrain(
                    &mut global_resource,
                    &mut track_resource,
                    &mut commands,
                    &mut meshes,
                    &mut terrain_material_asset,
                );
            };

            ui.allocate_space(egui::Vec2::new(0.0, 10.0));

            // new track element
            ui.heading("New Track Element");

            ui.allocate_space(egui::Vec2::new(0.0, 10.0));

            ui.horizontal(|ui| {
                if ui.add(egui::Button::new("Add track element")).clicked() {
                    let track = TrackElement2D {
                        curvature: ui_resource.new_track_element.curvature,
                        curve_angle: ui_resource.new_track_element.curve_angle,
                        start_bank_angle: ui_resource.new_track_element.start_bank_angle,
                        end_bank_angle: ui_resource.new_track_element.end_bank_angle,
                        width: ui_resource.new_track_element.width,
                        length: ui_resource.new_track_element.length,
                    };

                    track_resource
                        .track_list
                        .insert(ui_resource.new_index, track);

                    set_ui_resource(&mut ui_resource, &mut track_resource);
                }

                ui.add(
                    egui::DragValue::new(&mut ui_resource.new_index)
                        .clamp_range(0..=track_resource.track_list.len())
                        .prefix("at index:   "),
                );
            });

            ui.allocate_space(egui::Vec2::new(1.0, 5.0));

            // row 1
            ui.horizontal(|ui| {
                ui.allocate_space(egui::Vec2::new(30.0, 1.0));
                ui.add(
                    egui::DragValue::new(&mut ui_resource.new_track_element.curvature)
                        .clamp_range(-2..=2)
                        .prefix("Curvature:   "),
                );
                ui.add(
                    egui::DragValue::new(&mut ui_resource.new_track_element.curve_angle)
                        .clamp_range(-190..=190)
                        .prefix("Curve Angle:   "),
                );
                ui.add(
                    egui::DragValue::new(&mut ui_resource.new_track_element.length)
                        .clamp_range(0..=100)
                        .prefix("Length:   "),
                );
            });

            // row 2
            ui.horizontal(|ui| {
                ui.allocate_space(egui::Vec2::new(30.0, 1.0));
                ui.add(
                    egui::DragValue::new(&mut ui_resource.new_track_element.start_bank_angle)
                        .clamp_range(-60..=60)
                        .prefix("Start Bank Angle:   "),
                );
                ui.add(
                    egui::DragValue::new(&mut ui_resource.new_track_element.end_bank_angle)
                        .clamp_range(-60..=60)
                        .prefix("End Bank Angle:   "),
                );
            });

            ui.allocate_space(egui::Vec2::new(0.0, 10.0));

            // track list
            ui.heading("Track List");

            ui.allocate_space(egui::Vec2::new(0.0, 10.0));

            egui::ScrollArea::vertical().show(ui, |ui| {
                let mut index = 0;

                while index < track_resource.track_list.len() - 1 {
                    track_menu_item(ui, index, &mut global_resource, &mut track_resource);

                    index += 1;
                }
            });
        });
}

fn track_menu_item(
    ui: &mut Ui,
    index: usize,
    global_resource: &mut ResMut<GlobalResource>,
    track_resource: &mut ResMut<TrackResource>,
) {
    // track list
    let index_string = index.to_string();
    let label = String::from("Track Element ") + &index_string;

    ui.allocate_space(egui::Vec2::new(30.0, 1.0));

    ui.horizontal(|ui| {
        if ui
            .add(egui::SelectableLabel::new(
                index as i32 == global_resource.highlighted_track_index,
                &label,
            ))
            .clicked()
        {
            global_resource.highlighted_track_index = index as i32;
        };

        if ui.button("Delete").clicked() {
            track_resource.track_list.remove(index);
        };
        if ui.button("Move Up").clicked() {
            if index > 0 {
                track_resource.track_list.swap(index - 1, index);
            }
        }
        if ui.button("Move Down").clicked() {
            if index < track_resource.track_list.len() - 1 {
                track_resource.track_list.swap(index, index + 1);
            }
        };
    });

    ui.allocate_space(egui::Vec2::new(1.0, 5.0));

    // row 1
    ui.horizontal(|ui| {
        ui.allocate_space(egui::Vec2::new(30.0, 1.0));
        ui.add(
            egui::DragValue::new(&mut track_resource.track_list[index as usize].curvature)
                .clamp_range(-2..=2)
                .prefix("Curvature:   "),
        );
        ui.add(
            egui::DragValue::new(&mut track_resource.track_list[index as usize].curve_angle)
                .clamp_range(-190..=190)
                .prefix("Curve Angle:   "),
        );
        ui.add(
            egui::DragValue::new(&mut track_resource.track_list[index as usize].length)
                .clamp_range(0..=100)
                .prefix("Length:   "),
        );
    });

    // row 2
    ui.horizontal(|ui| {
        ui.allocate_space(egui::Vec2::new(30.0, 1.0));
        ui.add(
            egui::DragValue::new(&mut track_resource.track_list[index as usize].start_bank_angle)
                .clamp_range(-60..=60)
                .prefix("Start Bank Angle:   "),
        );
        ui.add(
            egui::DragValue::new(&mut track_resource.track_list[index as usize].end_bank_angle)
                .clamp_range(-60..=60)
                .prefix("End Bank Angle:   "),
        );
    });

    ui.allocate_space(egui::Vec2::new(1.0, 10.0));
}
