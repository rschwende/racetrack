use bevy::prelude::*;

use crate::components::*;

pub const TERRAIN_OFFSET: f32 = 25.; // how much the terrain extends past track

// sets initial values
pub fn set_global_resource(mut global_resource: ResMut<GlobalResource>) {
    // noise parameters
    global_resource.frequency_scale = 0.;
    global_resource.amplitude_scale = 0.;
    global_resource.octaves = 0;
    global_resource.lacunarity = 0.;
    global_resource.gain = 0.;

    // track parameters
    global_resource.x_min = -TERRAIN_OFFSET;
    global_resource.x_max = TERRAIN_OFFSET;
    global_resource.y_min = -TERRAIN_OFFSET;
    global_resource.y_max = TERRAIN_OFFSET;
}
