use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use shared::game::defs::{Defs, TowerRef};
use shared::game::shared_game::SharedGame;
use std::f32::consts::TAU;

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

pub fn debug_lines_tower(
    defs: Res<Defs>,
    game: Query<&SharedGame>,
    mut lines: ResMut<DebugLines>,
    query: Query<(&Transform, &TowerRef)>,
) {
    let game = match game.get_single() {
        Ok(g) => g,
        Err(_) => {
            warn!("Could not get game for debug_lines_path");
            return;
        }
    };

    for entity in game.entities.values() {
        let (transform, tower_ref) = match query.get(*entity) {
            Err(_) => continue,
            Ok(t) => t,
        };

        let tower = match defs.tower(tower_ref) {
            None => {
                warn!("5 Could not get tower for {:?}", tower_ref);
                continue;
            }
            Some(t) => t,
        };

        circle(
            &mut lines,
            transform.translation,
            tower.size,
            6,
            Color::BLACK,
        );
        circle(
            &mut lines,
            transform.translation,
            tower.range,
            8,
            Color::BLACK,
        );
    }
}

fn circle(lines: &mut ResMut<DebugLines>, pos: Vec3, rad: f32, steps: usize, color: Color) {
    for i in 0..steps {
        let a = i as f32 / steps as f32 * TAU;
        let b = (i + 1) as f32 / steps as f32 * TAU;
        lines.line_colored(
            pos + Vec3::new(a.cos(), 0.0, a.sin()) * rad,
            pos + Vec3::new(b.cos(), 0.0, b.sin()) * rad,
            0.0,
            color,
        );
    }
}
