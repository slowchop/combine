use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::Protocol"]
pub struct JoinFriendGame {
    pub name: Property<(u8, u8)>,
    pub game: Property<Option<u32>>,
}
