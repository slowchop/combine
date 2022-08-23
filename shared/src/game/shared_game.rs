use crate::game::owner::Owner;
use crate::game::player::SharedPlayer;
use crate::ticks::Ticks;
use crate::{RELEASE_CLOCK_TIME, TICKS_PER_DAY, TICKS_PER_SECOND};
use bevy_ecs::prelude::*;
use bevy_math::Vec2;
use bevy_utils::{HashMap, HashSet};
use naia_shared::serde::{BitReader, BitWrite, Serde, SerdeErr};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use server::path::Path;
use std::fmt::{Debug, Formatter};
use std::time::Duration;

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
    pub map: String,
    pub players: Vec<SharedPlayer>,
    pub spawn_points: HashMap<Owner, Vec2>,
    pub paths: HashMap<Owner, Path>,
    pub entities: HashMap<ServerEntityId, Entity>,

    ticks: Ticks,
}

impl Debug for SharedGame {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "SharedGame {{ map: {:?}, entities: {:?}, players: {:?} }}",
            self.map,
            self.entities.len(),
            self.players,
        )
    }
}

impl SharedGame {
    pub fn new(map: String, players: Vec<SharedPlayer>) -> Self {
        Self {
            map,
            entities: HashMap::with_capacity(1024),
            spawn_points: Default::default(),
            players,
            ticks: 0u64.into(),
            paths: Default::default(),
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

    pub fn tick(&mut self) {
        self.ticks += Ticks(1);
    }

    pub fn ticks(&self) -> Ticks {
        self.ticks
    }

    pub fn duration(&self) -> Option<Duration> {
        self.ticks().to_duration()
    }

    pub fn ticks_since_start_of_day(&self) -> Ticks {
        let v = self.ticks().0 % TICKS_PER_DAY.0;
        debug_assert!(v >= 0);
        Ticks(v)
    }

    pub fn start_of_day(&self) -> Ticks {
        Ticks(self.ticks().0 - self.ticks_since_start_of_day().0)
    }

    pub fn day(&self) -> i64 {
        (self.ticks / TICKS_PER_DAY).0
    }

    pub fn next_release_ticks(&self) -> Ticks {
        let next = Ticks(self.day() * TICKS_PER_DAY.0 + RELEASE_CLOCK_TIME.0);
        if self.ticks <= next {
            next
        } else {
            next + TICKS_PER_DAY
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ticks() {
        let next_release_time: Ticks = RELEASE_CLOCK_TIME;
        let mut g = SharedGame::new("".to_string(), vec![]);

        g.ticks = 0.into();
        assert_eq!(g.next_release_ticks(), next_release_time);
        g.ticks = next_release_time - 1.into();
        assert_eq!(g.next_release_ticks(), next_release_time);
        g.ticks = next_release_time;
        assert_eq!(g.next_release_ticks(), next_release_time);

        g.ticks = next_release_time + 1.into();
        assert_eq!(g.next_release_ticks(), next_release_time + TICKS_PER_DAY);
        g.ticks = TICKS_PER_DAY - Ticks(1);
        assert_eq!(g.next_release_ticks(), next_release_time + TICKS_PER_DAY);
        g.ticks = TICKS_PER_DAY;
        assert_eq!(g.next_release_ticks(), next_release_time + TICKS_PER_DAY);
        g.ticks = TICKS_PER_DAY + Ticks(1);
        assert_eq!(g.next_release_ticks(), next_release_time + TICKS_PER_DAY);
        g.ticks = TICKS_PER_DAY + next_release_time;
        assert_eq!(g.next_release_ticks(), next_release_time + TICKS_PER_DAY);

        g.ticks = TICKS_PER_DAY + next_release_time + 1.into();
        assert_eq!(
            g.next_release_ticks(),
            next_release_time + TICKS_PER_DAY + TICKS_PER_DAY
        );
    }
}
