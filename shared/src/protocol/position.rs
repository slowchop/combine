use bevy_ecs::prelude::Component;
use bevy_math::Vec2;

use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct Position {
    pub x: Property<f32>,
    pub y: Property<f32>,
}

impl Position {
    pub fn new(v: Vec2) -> Self {
        Position::new_complete(v.x, v.y)
    }

    pub fn vec2(&self) -> Vec2 {
        Vec2::new(*self.x, *self.y)
    }
}
