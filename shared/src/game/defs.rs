use bevy_ecs::prelude::Component;
use bevy_utils::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Defs {
    pub levels: Vec<String>,
    pub towers: HashMap<String, Tower>,
    pub creeps: HashMap<String, Creep>,
}

impl Defs {
    pub fn load() -> Self {
        let static_data_path = include_str!("../../../game/assets/defs.yaml");
        let static_data_str = std::fs::read_to_string(static_data_path).unwrap();
        serde_yaml::from_str(&static_data_str).unwrap()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tower {
    name: String,
    combo: Vec<String>,
    texture: String,
    damage: f32,
    range: f32,
}

#[derive(Component)]
pub struct TowerRef(String);

#[derive(Component, Debug, Clone, Deserialize)]
pub struct Creep {
    name: String,
}

#[derive(Component)]
pub struct CreepRef(String);
