use crate::game::defs::NetVec2;
use crate::game::shared_game::ServerEntityId;
use crate::Ticks;
use bevy_ecs::prelude::Component;
use bevy_math::Vec2;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct ReleaseCreep {
    pub server_entity_id: Property<ServerEntityId>,
    pub starting_position: Property<NetVec2>,
    pub starting_tick: Property<Ticks>,
}

impl ReleaseCreep {
    pub fn new(
        server_entity_id: ServerEntityId,
        starting_position: Vec2,
        starting_tick: Ticks,
    ) -> Self {
        ReleaseCreep::new_complete(server_entity_id, starting_position.into(), starting_tick)
    }
}
