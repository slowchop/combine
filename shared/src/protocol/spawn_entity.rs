use crate::game::defs::EntityDef;
use bevy_ecs::prelude::Component;
use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::protocol::Protocol"]
pub struct SpawnEntity {
    entity_def_data: Property<Vec<u8>>,
}

impl SpawnEntity {
    pub fn new(e: &EntityDef) -> Self {
        // let encoded = rmp_serde::to_vec(&e).unwrap();
        // let encoded = bincode::serialize(&e).unwrap();

        // TODO: DONT USE YAML
        let encoded = serde_yaml::to_string(&e).unwrap();

        SpawnEntity::new_complete(encoded.as_bytes().to_vec())
    }

    pub fn to_entity_def(&self) -> Result<EntityDef, serde_yaml::Error> {
        let encoded: &[u8] = self.entity_def_data.as_slice();

        serde_yaml::from_str(std::str::from_utf8(encoded).unwrap())

        // rmp_serde::from_slice(encoded)
        // bincode::deserialize(encoded)
    }
}
