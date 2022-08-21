use crate::game_info::GameInfo;
use bevy_utils::HashMap;
use naia_bevy_server::{RoomKey, UserKey};
use shared::player_name::PlayerName;
use std::collections::VecDeque;

pub struct State {
    pub main_room_key: RoomKey,
}

/// Next in queue is the first entry. Last to join is the last entry.
#[derive(Default)]
pub struct PlayerQueue(pub VecDeque<UserKey>);

impl PlayerQueue {
    pub fn add(&mut self, user_key: UserKey) {
        self.0.push_back(user_key);
    }

    pub fn pair(&mut self) -> Option<(UserKey, UserKey)> {
        if self.0.len() < 2 {
            return None;
        }

        let first = self.0.pop_front().unwrap();
        let second = self.0.pop_front().unwrap();
        Some((first, second))
    }
}

#[derive(Debug)]
pub struct Player {
    pub name: PlayerName,
}

pub struct PlayerInfo(pub HashMap<UserKey, Player>);

impl Default for PlayerInfo {
    fn default() -> Self {
        PlayerInfo(HashMap::new())
    }
}

pub struct Games(pub HashMap<RoomKey, GameInfo>);
impl Default for Games {
    fn default() -> Self {
        Games(HashMap::new())
    }
}
