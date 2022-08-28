use bevy_ecs::prelude::*;
use bevy_render::color::Color;
use naia_shared::serde::{BitReader, BitWrite, Serde, SerdeErr};
use serde::{Deserialize, Serialize};

// Yellow
pub const PLAYER_1_COLOR: Color = Color::rgb(0.937, 0.753, 0.435);

// Blue
pub const PLAYER_2_COLOR: Color = Color::rgb(0.262, 0.494, 0.866);

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

    pub fn color(&self) -> Color {
        match self.0 {
            0 => PLAYER_1_COLOR,
            1 => PLAYER_2_COLOR,
            _ => Color::ORANGE_RED,
        }
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
