use crate::game::owner::Owner;
use crate::game::player::SharedPlayer;
use bevy_ecs::prelude::*;
use bevy_math::Vec2;
use bevy_utils::{HashMap, HashSet};
use naia_shared::serde::{BitReader, BitWrite, Serde, SerdeErr};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(Component, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct ServerEntityId(pub u32);

impl Serde for ServerEntityId {
    fn ser(&self, writer: &mut dyn BitWrite) {
        self.0.ser(writer);
    }
    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        Ok(ServerEntityId(Serde::de(reader)?))
    }
}

#[derive(Component)]
pub struct SharedGame {
    map: String,
    entities: HashMap<ServerEntityId, Entity>,
    players: Vec<SharedPlayer>,
}

impl Debug for SharedGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SharedGame {{ map: {:?}, entities: {:?}, players: {:?} }}",
            self.map,
            self.entities.len(),
            self.players
        )
    }
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

    pub fn can_build_tower(&self, owner: &Owner, position: &Vec2, tower: &str) -> CanBuild {
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
