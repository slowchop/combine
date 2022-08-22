use crate::game::game_info::Owner;
use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};
use bevy_transform::prelude::Transform;
use bevy_utils::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

pub const PIXELS_PER_METER: f32 = 250.;

#[derive(Serialize, Deserialize)]
pub struct Defs {
    pub levels: HashMap<String, LevelDef>,
    pub towers: HashMap<String, Tower>,
    pub creeps: HashMap<String, Creep>,
    pub textures: HashMap<String, TextureDefinition>,
}

impl Defs {
    pub fn load() -> Self {
        let data = include_str!("../../../game/assets/defs.yaml");
        serde_yaml::from_str(data).unwrap()
    }

    // This has to be run from the game directory!
    pub fn save(&self) {
        let yaml = serde_yaml::to_string(&self).unwrap();
        let bytes = yaml.as_bytes();

        File::create("assets/defs.yaml")
            .unwrap()
            .write_all(bytes)
            .unwrap();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tower {
    name: String,
    combo: Vec<String>,
    texture: String,
    damage: f32,
    range: f32,
    cost: u32,
}

#[derive(Component)]
pub struct TowerRef(String);

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Creep {
    name: String,
    combo: Vec<String>,
    texture: String,
    speed: f32,
    cost: u32,
}

#[derive(Component)]
pub struct CreepRef(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureDefinition {
    pub size: Vec2,
}

#[derive(Serialize, Deserialize)]
pub struct LevelDef {
    pub name: String,
    pub entities: Vec<EntityDef>,
}

pub fn level_entity_transform(
    level_entity: &EntityDef,
    texture_def: &TextureDefinition,
) -> Option<Transform> {
    let position = level_entity.position?;
    let x = position.x;
    let y = position.y;
    Some(Transform::from_xyz(x, 0., y).with_scale(Vec3::new(
        texture_def.size[0] as f32 / PIXELS_PER_METER,
        texture_def.size[1] as f32 / PIXELS_PER_METER,
        1.0,
    )))
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EntityDef {
    pub texture: Option<String>,
    pub position: Option<Vec2>,
    #[serde(default, rename = "type")]
    pub entity_type: EntityType,
    pub owner: Option<Owner>,
    pub radius: Option<f32>,
    pub steps: Option<Vec<Vec2>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Sprite,
    Ground,
    Spawn,
    Base,
    Path,
}

impl Default for EntityType {
    fn default() -> Self {
        EntityType::Sprite
    }
}
