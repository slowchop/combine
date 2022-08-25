use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::ShaderRef;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "c690fdae-d598-45ab-8225-97e2a2f056e0"]
pub struct BillboardMaterial {
    #[uniform(0)]
    pub color: Color,

    // -1 no owner
    // 0 or 1 otherwise
    #[uniform(1)]
    pub owner: i32,

    #[texture(2)]
    #[sampler(3)]
    pub color_texture: Option<Handle<Image>>,
    pub alpha_mode: AlphaMode,
}

impl Material for BillboardMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/billboard.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }
}
