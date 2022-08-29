use bevy_ecs::prelude::*;
use std::time::Duration;

#[derive(Component, Debug)]
pub struct Speed(pub f32);

#[derive(Component, Debug)]
pub struct MaxHealth(pub u32);

#[derive(Component, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Tooltip {
    Combo(u8),
}
