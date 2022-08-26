use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use shared::game::defs::Defs;
use shared::game::shared_game::SharedGame;

pub fn debug_lines_path(game: Query<&SharedGame>, mut lines: ResMut<DebugLines>) {
    let game = match game.get_single() {
        Ok(g) => g,
        Err(_) => {
            warn!("Could not get game for debug_lines_path");
            return;
        }
    };

    for (owner, path) in &game.paths {
        for (a, b) in path.0.iter().zip(path.0.iter().skip(1)) {
            lines.line_colored(*a + Vec3::Y * 0.1, *b + Vec3::Y * 0.1, 0.0, owner.color());
        }
    }
}
