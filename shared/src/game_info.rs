use crate::player_name::PlayerName;
use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Room(u64);

/// 0 or 1.
#[derive(Component, Default)]
pub struct Owner(u8);

impl Owner {
    pub fn new(owner: u8) -> Self {
        if owner > 1 {
            panic!("Owner must be 0 or 1");
        }
        Self(owner)
    }
}

impl From<Owner> for u8 {
    fn from(o: Owner) -> Self {
        o.0
    }
}