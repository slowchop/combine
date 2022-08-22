use crate::game::defs::EntityDef;
use crate::game::towers::Tower;
use bevy_ecs::prelude::Component;
use bevy_math::Vec2;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct SpawnEntity {
    /// I couldn't use the lower level encoder/decoder because they were private.
    /// Because I don't have time for the jam, I'm just going to splat it with messagepack.
    data: Property<Vec<u8>>,
}

impl SpawnEntity {
    pub fn new(e: &EntityDef) -> Self {
        let encoded = rmp_serde::to_vec(&e).unwrap();
        SpawnEntity::new_complete(encoded)
    }

    pub fn to_entity_def(&self) -> Result<EntityDef, rmp_serde::decode::Error> {
        rmp_serde::from_slice(&self.data)
    }
}
