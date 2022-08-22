use bevy_ecs::prelude::Component;
use bevy_math::Vec2;
use bevy_utils::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize)]
pub struct Defs {
    pub level: HashMap<String, Level>,
    pub towers: HashMap<String, Tower>,
    pub creeps: HashMap<String, Creep>,
    pub textures: HashMap<String, TextureDefinition>,
}

impl Defs {
    pub fn load() -> Self {
        let static_data_path = include_str!("../../../game/assets/defs.yaml");
        let static_data_str = std::fs::read_to_string(static_data_path).unwrap();
        serde_yaml::from_str(&static_data_str).unwrap()
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
}

#[derive(Component)]
pub struct TowerRef(String);

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Creep {
    name: String,
}

#[derive(Component)]
pub struct CreepRef(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureDefinition {
    pub size: Vec2,
}
