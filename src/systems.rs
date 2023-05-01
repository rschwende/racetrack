use crate::GlobalState;
use bevy::prelude::*;

pub const TERRAIN_OFFSET: f32 = 25.; // how much the terrain extends past track

pub fn set_global_state(mut global_state: ResMut<GlobalState>) {
    global_state.frequency_scale = 0.;
    global_state.amplitude_scale = 0.;
    global_state.octaves = 0;
    global_state.lacunarity = 0.;
    global_state.gain = 0.;

    // track parameters
    global_state.x_min = -TERRAIN_OFFSET;
    global_state.x_max = TERRAIN_OFFSET;
    global_state.y_min = -TERRAIN_OFFSET;
    global_state.y_max = TERRAIN_OFFSET;
}
