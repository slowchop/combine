use bevy_ecs::entity::Entity;
use naia_bevy_server::{RoomKey, UserKey};
use std::collections::HashMap;

pub struct State {
    pub main_room_key: RoomKey,
    // pub user_to_prediction_map: HashMap<UserKey, Entity>,
    // pub player_last_command: HashMap<Entity, KeyCommand>,
}
