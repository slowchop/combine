use crate::Ticks;
use bevy_ecs::prelude::*;
use bevy_math::{Vec2, Vec3};
use std::time::Duration;

#[derive(Component, Debug, Clone)]
pub struct Path(pub Vec<Vec3>);

#[derive(Component, Debug)]
pub struct PathProgress {
    // pub previous_position: Vec3,
    // pub previous_position_time: f64,
    pub target_position: Vec3,
    pub current_path_target: usize,
}

/// Only start moving after this time.
#[derive(Component, Debug)]
pub struct PathLeaveAt(pub Duration);
