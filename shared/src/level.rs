use crate::game_info::Owner;
use bevy_math::Vec2;
use bevy_utils::HashMap;
use serde::{Deserialize, Serialize};

pub struct ManagedLevel {
    pub entities: Vec<LevelEntity>,
    pub texture: HashMap<String, TextureDefinition>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureDefinition {
    pub path: String,
    pub size: [u32; 2],
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LevelEntity {
    pub texture: String,
    pub position: Vec2,
    pub test: Option<Vec2>,
    #[serde(default, rename = "type")]
    pub entity_type: EntityType,

    pub owner: Option<Owner>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Sprite,
    Ground,
}

impl Default for EntityType {
    fn default() -> Self {
        EntityType::Sprite
    }
}
