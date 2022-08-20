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
    #[texture(1)]
    #[sampler(2)]
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
