pub mod defs;
pub mod owner;
pub mod player;
pub mod shared_game;
pub mod towers;

use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
pub struct SpawnPoint;
