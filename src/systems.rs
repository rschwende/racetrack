use bevy::{
    prelude::*,
    render::{
        render_resource::{AddressMode, FilterMode, SamplerDescriptor},
        texture::ImageSampler,
    },
};

use crate::components::*;

pub const TERRAIN_OFFSET: f32 = 25.; // how much the terrain extends past track
pub const MAX_TRACK_HEIGHT: f32 = 5.; // positions secondary camera for track height map
pub const MIN_TRACK_HEIGHT: f32 = -2.; // gives a bottom depth for track height map

// sets initial values
pub fn set_global_resource(mut global_resource: ResMut<GlobalResource>) {
    // noise parameters
    global_resource.frequency_scale = 0.;
    global_resource.amplitude_scale = 0.;
    global_resource.octaves = 0;
    global_resource.lacunarity = 0.;
    global_resource.gain = 0.;
    global_resource.scale = 1.;

    // track parameters
    global_resource.x_min = -TERRAIN_OFFSET;
    global_resource.x_max = TERRAIN_OFFSET;
    global_resource.y_min = -TERRAIN_OFFSET;
    global_resource.y_max = TERRAIN_OFFSET;
    global_resource.max_track_ht = MAX_TRACK_HEIGHT;
    global_resource.min_track_ht = MIN_TRACK_HEIGHT;

    global_resource.show_track_map = false;
    global_resource.track_texture_scale = 1.;
    global_resource.terrain_texture_scale = 1.;
}

pub fn load_assets(asset_server: Res<AssetServer>, mut track_resource: ResMut<TrackResource>) {
    // https://www.freepik.com/free-photo/asphalt-road-texture-dark-gray-color_5247630.htm#query=asphalt%20texture%20seamless&position=0&from_view=keyword&track=ais Image by starline</a> on Freepik
    track_resource.track_texture_handle = asset_server.load("textures/asphalt-road-texture.png");
}

pub fn change_texture_mode(
    mut done: Local<bool>,
    track_resource: ResMut<TrackResource>,
    mut images: ResMut<Assets<Image>>,
) {
    if *done {
        return;
    }

    // setting texture address mode to mirror repeat
    if let Some(image) = images.get_mut(&track_resource.track_texture_handle) {
        image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
            label: Some("Track Texture"),
            address_mode_u: AddressMode::MirrorRepeat,
            address_mode_v: AddressMode::MirrorRepeat,
            address_mode_w: AddressMode::MirrorRepeat,
            mipmap_filter: FilterMode::Linear,
            ..default()
        });
        *done = true;
    };
}

pub fn set_visibility(
    mut vis_query: Query<(
        &mut Visibility,
        Option<&TrackElement>,
        Option<&PlaneElement>,
        Option<&TerrainElement>,
    )>,
    global_resource: Res<GlobalResource>,
) {
    for (mut curr_vis, track, plane, terrain) in vis_query.iter_mut() {
        // track
        if track.is_some() {
            if global_resource.show_track {
                *curr_vis = Visibility::Visible;
            } else {
                *curr_vis = Visibility::Hidden;
            }
        }

        // track map texture
        if plane.is_some() {
            if global_resource.show_track_texture {
                *curr_vis = Visibility::Visible;
            } else {
                *curr_vis = Visibility::Hidden;
            }
        }

        // terrain
        if terrain.is_some() {
            if global_resource.show_terrain {
                *curr_vis = Visibility::Visible;
            } else {
                *curr_vis = Visibility::Hidden;
            }
        }
    }
}

pub fn despawn(commands: &mut Commands, query: &mut Query<Entity, With<MyEntity>>) {
    for track in query {
        commands.entity(track).despawn();
    }
}

pub fn directional_light(mut commands: Commands) {
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
