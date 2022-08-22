use crate::game::managed_game::{LevelEntity, TextureDefinition};
use bevy_math::Vec3;
use bevy_transform::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

pub const PIXELS_PER_METER: f32 = 250.;

pub struct YamlLevel {
    pub name: String,
    pub entities: Vec<LevelEntity>,
}

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
