use crate::states::playing::spawn_entities::{HasColdEffect, HasFireEffect};
use bevy::prelude::*;

#[derive(Component, Debug)]
pub enum CurrentEffect {
    None,
    Fire,
    Cold,
}

pub fn effect_bubbles(
    time: Res<Time>,
    query: Query<(&mut CurrentEffect, &HasColdEffect, &HasFireEffect)>,
) {
    for (current_effect, cold, fire) in query.iter() {
        // if cold.until {}
        //
    }
}
