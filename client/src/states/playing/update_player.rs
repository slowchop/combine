use bevy::prelude::*;
use shared::game::owner::Owner;
use shared::game::shared_game::SharedGame;

#[derive(Debug)]
pub struct UpdatePlayerEvent {
    pub owner: Owner,
    pub gold: u32,
    pub lives: u32,
}

pub fn update_player(
    mut update_player_events: EventReader<UpdatePlayerEvent>,
    mut game: Query<&mut SharedGame>,
) {
    for update_player_event in update_player_events.iter() {
        let mut game = if let Ok(g) = game.get_single_mut() {
            g
        } else {
            warn!("No game when updating player: {:?}", update_player_event);
            continue;
        };

        let mut player = if let Some(p) = game.get_player_mut(update_player_event.owner) {
            p
        } else {
            warn!("No player when updating player: {:?}", update_player_event);
            continue;
        };

        player.gold = update_player_event.gold;
        player.lives = update_player_event.lives;
        dbg!(player);
    }
}
