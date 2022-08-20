use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::Protocol"]
pub struct Auth {
    pub name: Property<(u8, u8)>,
}

impl Auth {
    pub fn new() -> Self {
        Auth::new_complete((0, 0))
    }
}
