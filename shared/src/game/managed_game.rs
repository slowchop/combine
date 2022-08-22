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
    pub players: Vec<Player>,
}

impl ManagedGame {
    pub fn new(players: Vec<Player>) -> Self {
        Self {
            entities: HashSet::with_capacity(1024),
            players,
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
