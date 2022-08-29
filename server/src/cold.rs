use crate::creeps::ColdEffect;
use bevy_ecs::prelude::*;
use shared::game::position::Velocity;
use shared::game::shared_game::ServerEntityId;

pub fn monitor_cold_changes(changed: Query<(&ServerEntityId, &ColdEffect), Changed<ColdEffect>>) {
    // for s in changed.iter() {
    //     println!("Cold effect changed: {:?}", s);
    // }
}
