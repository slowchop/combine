use crate::game::defs::TowerRef;
use crate::game::owner::Owner;
use crate::game::path::Path;
use crate::game::player::SharedPlayer;
use crate::ticks::Ticks;
use crate::{RELEASE_CLOCK_TIME, RESPAWN_CLOCK_TIME, TICKS_PER_DAY, TICKS_PER_SECOND};
use bevy_ecs::prelude::*;
use bevy_math::Vec2;
use bevy_utils::{HashMap, HashSet};
use naia_shared::serde::{BitReader, BitWrite, Serde, SerdeErr};
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::slice::Iter;
use std::time::Duration;
use tracing::warn;

#[derive(
    Component, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize,
)]
pub struct ServerEntityId(pub u32);

impl ServerEntityId {
    pub fn random() -> Self {
        let mut rng = thread_rng();
        Self(rng.gen())
    }
}

impl Serde for ServerEntityId {
    fn ser(&self, writer: &mut dyn BitWrite) {
        self.0.ser(writer);
    }
    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        Ok(ServerEntityId(Serde::de(reader)?))
    }
}

pub struct Multiplier {
    pub speed: f32,
    pub health: f32,
}

pub enum TimeLeft {
    ReleaseCreeps(Duration),
    RespawnCreeps(Duration),
}

impl TimeLeft {
    pub fn duration(&self) -> Duration {
        match self {
            TimeLeft::ReleaseCreeps(d) => *d,
            TimeLeft::RespawnCreeps(d) => *d,
        }
    }
}

#[derive(Component)]
pub struct SharedGame {
    pub map: String,
    pub players: Vec<SharedPlayer>,
    pub spawn_points: HashMap<Owner, Vec2>,
    pub paths: HashMap<Owner, Path>,
    pub entities: HashMap<ServerEntityId, Entity>,
    pub winner: Option<Owner>,
    pub round: u32,
    // pub seconds_left: SecondsLeft,
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
            winner: None,
            round: 0,
        }
    }

    pub fn multipliers(&self) -> Multiplier {
        Multiplier {
            speed: (self.round as f32) * 0.3 + 1.0 + ((self.round as f32).powf(2.1)) * 0.1,
            health: (self.round as f32) * 0.3 + 1.0 + ((self.round as f32).powf(1.8)) * 0.1,
        }
    }

    pub fn free_server_id(&mut self) -> ServerEntityId {
        loop {
            let id = ServerEntityId(thread_rng().gen());
            if self.entities.contains_key(&id) {
                continue;
            }
            return id;
        }
    }

    pub fn server_add_entity(&mut self, entity: Entity) -> ServerEntityId {
        let id = self.free_server_id();
        self.entities.insert(id, entity);
        id
    }

    pub fn get_player(&self, owner: Owner) -> Option<&SharedPlayer> {
        self.players.iter().find(|p| p.owner == owner)
    }

    pub fn get_player_mut(&mut self, owner: Owner) -> Option<&mut SharedPlayer> {
        self.players.iter_mut().find(|p| p.owner == owner)
    }

    pub fn client_add_entity(&mut self, server_entity_id: ServerEntityId, entity: Entity) {
        if self.entities.contains_key(&server_entity_id) {
            warn!("Tried to insert the same server entity twice");
        }
        self.entities.insert(server_entity_id, entity);
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

    pub fn next_respawn_ticks(&self) -> Ticks {
        let next = Ticks(self.day() * TICKS_PER_DAY.0 + RESPAWN_CLOCK_TIME.0);
        if self.ticks <= next {
            next
        } else {
            next + TICKS_PER_DAY
        }
    }

    pub fn time_left(&self) -> TimeLeft {
        let release = self.next_release_ticks();
        let respawn = self.next_respawn_ticks();
        if self.ticks < release {
            TimeLeft::ReleaseCreeps((release - self.ticks).to_duration().unwrap())
        } else if self.ticks < respawn {
            TimeLeft::RespawnCreeps((respawn - self.ticks).to_duration().unwrap())
        } else {
            TimeLeft::ReleaseCreeps(Duration::default())
        }
    }
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
