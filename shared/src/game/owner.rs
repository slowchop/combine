use crate::game::player::PlayerName;
use bevy_ecs::prelude::*;
use naia_shared::serde::{BitReader, BitWrite, Serde, SerdeErr};
use serde::{Deserialize, Serialize};

/// 0 or 1.
#[derive(Component, Default, Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Owner(pub u8);

impl Owner {
    pub fn new(owner: u8) -> Self {
        if owner > 1 {
            panic!("Owner must be 0 or 1");
        }
        Self(owner)
    }

    pub fn other_player(&self) -> Owner {
        Owner(1 - self.0)
    }

    // Bit of a hack!
    pub fn waiting() -> Self {
        Self(42)
    }
}

impl From<Owner> for u8 {
    fn from(o: Owner) -> Self {
        o.0
    }
}

impl Serde for Owner {
    fn ser(&self, writer: &mut dyn BitWrite) {
        self.0.ser(writer);
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        Ok(Owner(Serde::de(reader)?))
    }
}
