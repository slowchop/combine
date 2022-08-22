use crate::game_info::ServerGameInfo;
use crate::server_player::ServerPlayer;
use bevy_utils::HashMap;
use naia_bevy_server::{RoomKey, UserKey};
use shared::game::player_name::PlayerName;
use std::collections::VecDeque;

pub struct Global {
    pub main_room_key: RoomKey,
}

/// Next in queue is the first entry. Last to join is the last entry.
#[derive(Default)]
pub struct PlayerQueue(pub VecDeque<UserKey>);

impl PlayerQueue {
    pub fn add(&mut self, user_key: UserKey) {
        self.0.push_back(user_key);
    }

    pub fn pair(&mut self) -> Option<[UserKey; 2]> {
        if self.0.len() < 2 {
            return None;
        }

        let first = self.0.pop_front().unwrap();
        let second = self.0.pop_front().unwrap();
        Some([first, second])
    }
}

pub struct Players(pub HashMap<UserKey, ServerPlayer>);

impl Default for Players {
    fn default() -> Self {
        Players(HashMap::new())
    }
}

impl Players {
    pub fn set_room(&mut self, user_key: &UserKey, room_key: RoomKey) {
        let mut player = self.0.get_mut(user_key).unwrap();
        player.room = room_key;
    }
}

pub struct Games(pub HashMap<RoomKey, ServerGameInfo>);

impl Default for Games {
    fn default() -> Self {
        Games(HashMap::new())
    }
}
