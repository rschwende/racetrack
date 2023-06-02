use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::{AsBindGroup, ShaderRef},
};

// Material that will be used on terrain plane
#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "c829f840-00e9-11ee-be56-0242ac120002"]
pub struct TextureMaterial {}

impl Material for TextureMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/texture_shader.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/texture_shader.wgsl".into()
    }
}
