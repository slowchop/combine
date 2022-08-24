use bevy::prelude::*;
use shared::game::shared_game::SharedGame;

pub fn add_ticks_to_game(mut game: Query<&mut SharedGame>) {
    let mut game = if let Ok(g) = game.get_single_mut() {
        g
    } else {
        return;
    };

    game.tick();
}
