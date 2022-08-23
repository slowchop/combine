use bevy_ecs::prelude::*;

#[derive(Component, Debug)]
pub struct Health(pub u32);

#[derive(Component, Debug)]
pub struct Damage(pub f32);

#[derive(Component, Debug)]
pub struct Range(pub f32);

#[derive(Component, Debug)]
pub struct Speed(pub f32);
