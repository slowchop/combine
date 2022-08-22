use crate::game::owner::Owner;
use crate::game::player::SharedPlayer;
use crate::game::towers::Tower;
use bevy_ecs::prelude::*;
use bevy_math::Vec2;
use bevy_utils::{HashMap, HashSet};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ServerEntityId(pub u32);

#[derive(Component)]
pub struct SharedGame {
    map: String,
    entities: HashMap<ServerEntityId, Entity>,
    players: Vec<SharedPlayer>,
}

impl SharedGame {
    pub fn new(map: String, players: Vec<SharedPlayer>) -> Self {
        Self {
            map,
            entities: HashMap::with_capacity(1024),
            players,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> ServerEntityId {
        loop {
            let id = ServerEntityId(thread_rng().gen());
            if self.entities.contains_key(&id) {
                continue;
            }
            self.entities.insert(id.clone(), entity);
            return id;
        }
    }

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
