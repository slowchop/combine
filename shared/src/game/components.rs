use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
pub struct Speed(pub f32);

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tooltip {
    Combo(u8),
}
