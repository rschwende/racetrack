use bevy::prelude::*;

#[derive(Component)]
pub struct TrackElement2D {
    pub curvature: f32,
    pub curve_angle: f32,
    pub width: f32,
    pub length: f32,
}

