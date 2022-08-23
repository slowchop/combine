pub mod components;
pub mod defs;
pub mod owner;
pub mod player;
pub mod shared_game;
pub mod systems;
pub mod position;

use crate::game::owner::Owner;
use crate::game::player::SharedPlayer;
use bevy_ecs::prelude::*;
use naia_shared::serde::{BitReader, BitWrite, Serde, SerdeErr};

#[derive(Component, Debug)]
pub struct SpawnPoint;

#[derive(Component, Clone, PartialEq, Debug)]
pub struct ClientGameInfo {
    pub map: String,
    pub players: Vec<SharedPlayer>,
    pub i_am: Owner,
}

impl Serde for ClientGameInfo {
    fn ser(&self, writer: &mut dyn BitWrite) {
        self.map.ser(writer);
        self.players.ser(writer);
        self.i_am.ser(writer);
    }
    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        Ok(ClientGameInfo {
            map: Serde::de(reader)?,
            players: Serde::de(reader)?,
            i_am: Serde::de(reader)?,
        })
    }
}
