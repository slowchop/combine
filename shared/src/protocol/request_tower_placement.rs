use bevy_ecs::prelude::Component;
use bevy_math::Vec2;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct RequestTowerPlacement {
    /// Contains a u64 that the client generates.
    ///
    /// This is used for the client to place the building instantly, and then removing it or
    /// updating it when the server sends it back.
    ///
    /// Server might respond with a denial so the client can remove the building with a message.
    pub placeholder: Property<u64>,

    /// Vec2
    pub x: Property<f32>,
    pub y: Property<f32>,

    pub tower: Property<String>,
}

impl RequestTowerPlacement {
    pub fn new(p: Vec2, tower: &str, placeholder: u64) -> Self {
        RequestTowerPlacement::new_complete(placeholder, p.x, p.y, tower.to_string())
    }

    pub fn position(&self) -> Vec2 {
        Vec2::new(*self.x, *self.y)
    }

    pub fn placeholder(&self) -> u64 {
        *self.placeholder
    }

    pub fn tower(&self) -> String {
        (*self.tower).clone()
    }
}
