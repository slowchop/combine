use bevy_ecs::prelude::*;
use bevy_math::{Vec2, Vec3};

#[derive(Component, Debug)]
pub struct Position(pub Vec3);

impl From<Vec2> for Position {
    fn from(v: Vec2) -> Self {
        Position(vec2_to_vec3(&v))
    }
}

pub fn vec2_to_vec3(v: &Vec2) -> Vec3 {
    Vec3::new(v.x, 0.0, v.y)
}

pub fn vec3_to_vec2(v: &Vec3) -> Vec2 {
    Vec2::new(v.x, v.z)
}

#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);
