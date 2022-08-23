use crate::Ticks;
use bevy_ecs::prelude::*;
use bevy_math::Vec2;

#[derive(Component, Debug)]
pub struct Path(Vec<Vec2>);

#[derive(Component, Debug)]
pub struct PathProgress {
    current_path: usize,
    started_at: f32,
    distance_to_next: f32,
}
