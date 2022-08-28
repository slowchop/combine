use crate::app::MyRaycastSet;
use crate::states::playing::floaty_text::{floaty_text_bundle, FONT};
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use shared::game::defs::{CreepRef, Defs, TowerRef};
use shared::game::owner::Owner;
use shared::game::position::vec2_to_vec3;
use shared::game::ClientGameInfo;

#[derive(Component)]
pub struct HoverStats;

#[derive(Debug)]
struct Closest {
    thing: TowerOrCreep,
    owner: Owner,
}

#[derive(Debug)]
enum TowerOrCreep {
    Tower(TowerRef),
    Creep(CreepRef),
}

pub fn hover_stats(
    defs: Res<Defs>,
    mut hover_stats: Query<(&mut Text, &mut Style), With<HoverStats>>,
    client_game_info: Query<&ClientGameInfo>,
    raycast_query: Query<&Intersection<MyRaycastSet>>,
    hoverable_query: Query<(&Transform, &Owner, Option<&TowerRef>, Option<&CreepRef>)>,
) {
    let client_game_info = client_game_info.single();

    let mut position = None;
    for intersection in raycast_query.iter() {
        let intersection = if let Some(i) = intersection.position() {
            i
        } else {
            continue;
        };
        position = Some(Vec2::new(intersection.x, intersection.z));
    }
    let mouse_position_vec2 = if let Some(position) = position {
        position
    } else {
        return;
    };
    let mouse_position_vec3 = vec2_to_vec3(&mouse_position_vec2);

    let mut closest = None;
    for (transform, owner, maybe_tower, maybe_creep) in hoverable_query.iter() {
        let min_distance = 2.0;
        let distance = (transform.translation - mouse_position_vec3).length();
        if distance < min_distance {
            closest = if let Some(tower_ref) = maybe_tower {
                Some(Closest {
                    thing: TowerOrCreep::Tower(tower_ref.clone()),
                    owner: *owner,
                })
            } else if let Some(creep_ref) = maybe_creep {
                Some(Closest {
                    thing: TowerOrCreep::Creep(creep_ref.clone()),
                    owner: *owner,
                })
            } else {
                None
            };
        }
    }

    for (mut text, mut style) in hover_stats.get_single_mut() {
        match &closest {
            Some(closest) => {
                let (title, other) = match &closest.thing {
                    TowerOrCreep::Tower(tower_ref) => {
                        let tower = defs.towers.get(tower_ref).unwrap();
                        (
                            tower.title.clone(),
                            format!("Damage: {}\nRange: {}\n", tower.instant_damage, tower.range),
                        )
                    }
                    TowerOrCreep::Creep(creep_ref) => {
                        let creep = defs.creeps.get(creep_ref).unwrap();
                        (creep.title.clone(), format!("Health: {}", creep.health))
                    }
                };
                text.sections[0].value = format!("{}\n", title);

                text.sections[1].style.color = client_game_info.i_am.color();
                if client_game_info.i_am == closest.owner {
                    text.sections[1].value = "This is yours!\n".to_string();
                } else {
                    text.sections[1].value = "The baddie owns this.\n".to_string();
                }

                text.sections[2].value = other;

                style.display = Display::Flex;
            }
            None => style.display = Display::None,
        }
    }
}
