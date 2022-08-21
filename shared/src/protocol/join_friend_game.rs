use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct JoinFriendGame {
    pub name: Property<String>,
    pub game: Property<Option<u32>>,
}
