use crate::game::game_info::Owner;
use crate::game::player::Player;
use crate::game::towers::Tower;
use bevy_ecs::prelude::*;
use bevy_math::Vec2;
use bevy_utils::{HashMap, HashSet};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ManagedEntityId(pub u32);

#[derive(Component)]
pub struct ManagedGame {
    pub entities: HashSet<Entity>,
    pub players: [Player; 2],
}

impl ManagedGame {
    fn new(players: [Player; 2]) -> Self {
        Self {
            entities: HashSet::with_capacity(1024),
            players,
        }
    }

    // fn new_id(&self) -> ManagedEntityId {
    //     loop {
    //         let id = thread_rng().gen();
    //         if self.entities.contains_key(&ManagedEntityId(id)) {
    //             continue;
    //         }
    //         return ManagedEntityId(id);
    //     }
    // }

    // pub fn from_players_level_textures(
    //     players: [Player; 2],
    //     level_entities: &[LevelEntity],
    // ) -> Self {
    //     let mut managed_level = ManagedGame::new(players);
    //
    //     for e in level_entities.into_iter() {
    //         let m = RuntimeEntity {
    //             id: managed_level.new_id(),
    //             definition: e.clone(),
    //         };
    //         managed_level.entities.insert(m.id.clone(), m);
    //     }
    //
    //     managed_level
    // }

    pub fn can_build_tower(&self, owner: &Owner, position: &Vec2, tower: &Tower) -> CanBuild {
        CanBuild::Yes
        // for (id, entity) in &self.entities {
        //     entity.definition.
        //
        // }
        // true
    }
}

pub enum CanBuild {
    Yes,
    No(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LevelEntity {
    pub texture: String,
    pub position: Vec2,
    #[serde(default, rename = "type")]
    pub entity_type: EntityType,
    pub owner: Option<Owner>,
    pub radius: Option<f32>,
}

pub struct RuntimeEntity {
    pub id: ManagedEntityId,
    pub radius: f32,
    pub definition: LevelEntity,
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
