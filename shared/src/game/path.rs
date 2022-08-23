use crate::Ticks;
use bevy_ecs::prelude::*;
use bevy_math::Vec2;

#[derive(Component, Debug)]
pub struct Path(pub Vec<Vec2>);

#[derive(Component, Debug)]
pub struct PathProgress {
    previous_position: Vec2,
    previous_ticks: Ticks,
    target_position: Vec2,
    current_path_target: usize,
}
