use bevy_ecs::prelude::*;
use bevy_utils::HashMap;
use naia_bevy_server::{RoomKey, UserKey};
use rand::{thread_rng, Rng};
use shared::game::owner::Owner;
use shared::game::player::PlayerName;
use shared::game::player::SharedPlayer;
use shared::game::shared_game::SharedGame;
use std::collections::VecDeque;

/// Next in queue is the first entry. Last to join is the last entry.
#[derive(Default)]
pub struct PlayerQueue(pub VecDeque<(UserKey, PlayerName)>);

impl PlayerQueue {
    pub fn add(&mut self, user_key: UserKey, player_name: PlayerName) {
        self.0.push_back((user_key, player_name));
    }

    pub fn find(&mut self, count: usize) -> Option<Vec<(UserKey, PlayerName)>> {
        if self.0.len() < count {
            return None;
        }

        let mut found = Vec::new();
        for _ in 0..count {
            let uk_pn = self.0.pop_front().unwrap();
            found.push(uk_pn);
        }
        Some(found)
    }
}

#[derive(Component, Default, Debug, Clone, Copy, Hash, PartialOrd, PartialEq, Eq)]
pub struct GameId(pub u32);

#[derive(Default)]
pub struct GameLookup(pub HashMap<GameId, SharedGame>);

#[derive(Default)]
pub struct GameUserLookup {
    game_to_users: HashMap<GameId, Vec<UserKey>>,
    user_to_game: HashMap<UserKey, (GameId, Owner)>,
}

impl GameUserLookup {
    pub fn new_game_id(&mut self) -> GameId {
        loop {
            let id = GameId(thread_rng().gen());
            if self.game_to_users.contains_key(&id) {
                continue;
            }
            return id;
        }
    }

    pub fn create_game_reference(&mut self, user_keys_and_owners: Vec<(UserKey, Owner)>) -> GameId {
        let game_id = self.new_game_id();
        self.game_to_users.insert(
            game_id,
            user_keys_and_owners.iter().map(|uk_o| uk_o.0).collect(),
        );

        for (user_key, owner) in user_keys_and_owners.iter() {
            self.user_to_game
                .insert(user_key.clone(), (game_id, owner.clone()));
        }
        game_id
    }

    pub fn get_user_game_and_owner(&self, user_key: &UserKey) -> Option<&(GameId, Owner)> {
        self.user_to_game.get(user_key)
    }

    pub fn get_game_users(&self, game_id: &GameId) -> Option<&Vec<UserKey>> {
        self.game_to_users.get(game_id)
    }
}
