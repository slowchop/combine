use bevy_ecs::prelude::*;
use bevy_utils::HashMap;
use naia_bevy_server::{RoomKey, UserKey};
use rand::{thread_rng, Rng};
use shared::game::player::PlayerName;
use shared::game::player::SharedPlayer;
use shared::game::shared_game::SharedGame;
use std::collections::VecDeque;

/// Next in queue is the first entry. Last to join is the last entry.
#[derive(Default)]
pub struct PlayerQueue(pub VecDeque<UserKey>);

impl PlayerQueue {
    pub fn add(&mut self, user_key: UserKey) {
        self.0.push_back(user_key);
    }

    pub fn find(&mut self, count: usize) -> Option<Vec<UserKey>> {
        if self.0.len() < count {
            return None;
        }

        let mut found = Vec::new();
        for _ in 0..count {
            let player = self.0.pop_front().unwrap();
            found.push(player);
        }
        Some(found)
    }
}

#[derive(Default)]
pub struct PlayerLookup(pub HashMap<UserKey, SharedPlayer>);

#[derive(Component, Default, Debug, Clone, Copy, Hash, PartialOrd, PartialEq, Eq)]
pub struct GameId(u32);

#[derive(Default)]
pub struct GameUserLookup {
    game_to_user: HashMap<GameId, Vec<UserKey>>,
    user_to_game: HashMap<UserKey, GameId>,
}

impl GameUserLookup {
    pub fn new_game_id(&mut self) -> GameId {
        loop {
            let id = GameId(thread_rng().gen());
            if self.game_to_user.contains_key(&id) {
                continue;
            }
            return id;
        }
    }

    pub fn create_game_reference(&mut self, user_keys: Vec<UserKey>) -> GameId {
        let game_id = self.new_game_id();
        self.game_to_user.insert(game_id, user_keys.clone());
        for user_key in user_keys.iter() {
            self.user_to_game.insert(user_key.clone(), game_id);
        }
        game_id
    }

    pub fn get_player_game(&self, user_key: &UserKey) -> Option<&GameId> {
        self.user_to_game.get(user_key)
    }

    pub fn get_game_players(&self, game_id: &GameId) -> Option<&Vec<UserKey>> {
        self.game_to_user.get(game_id)
    }
}

#[derive(Default)]
pub struct GameLookup(pub HashMap<GameId, SharedGame>);
