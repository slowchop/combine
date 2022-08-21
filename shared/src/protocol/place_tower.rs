use bevy_ecs::prelude::Component;
use bevy_math::Vec2;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct PlaceTower {
    pub x: Property<f32>,
    pub y: Property<f32>,
}

impl From<Vec2> for PlaceTower {
    fn from(v: Vec2) -> Self {
        PlaceTower::new_complete(v.x, v.y)
    }
}

impl From<PlaceTower> for Vec2 {
    fn from(place_tower: PlaceTower) -> Self {
        Vec2::new(*place_tower.x, *place_tower.y)
    }
}
