use crate::shader::BillboardMaterial;
use crate::states::playing::bottom_quad::BottomQuad;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};
use shared::game_info::Owner;
use shared::level::{LevelEntity, TextureDefinition};
use shared::player_name::PlayerName;
use std::f32::consts::TAU;

pub const PIXELS_PER_METER: f32 = 250.;

#[derive(Serialize, Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "3d95a211-1b29-44a3-a9db-875cf44ff92c"]
pub struct YamlLevel {
    pub name: String,
    pub entities: Vec<LevelEntity>,
}

#[derive(Debug, Serialize, Deserialize, TypeUuid)]
#[uuid = "f1235a5a-89e5-463f-b531-d193e1a63870"]
pub struct Textures(pub Vec<TextureDefinition>);

pub fn level_entity_transform(
    level_entity: &LevelEntity,
    texture_def: &TextureDefinition,
) -> Transform {
    let x = level_entity.position[0];
    let y = level_entity.position[1];
    Transform::from_xyz(x, 0., y).with_scale(Vec3::new(
        texture_def.size[0] as f32 / PIXELS_PER_METER,
        texture_def.size[1] as f32 / PIXELS_PER_METER,
        1.0,
    ))
}

#[derive(Debug, Eq, PartialEq)]
pub enum LevelLoadState {
    Loading,
    Loaded,
}
