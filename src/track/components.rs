use bevy::prelude::*;

#[derive(Component)]
pub struct Point {
    pub p: Vec3,
    pub n: Vec3,
    pub uv: Vec2,
}
