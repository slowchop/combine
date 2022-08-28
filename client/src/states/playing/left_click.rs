use crate::app::MyRaycastSet;
use crate::states::playing::creeps::Released;
use crate::states::playing::floaty_text::{floaty_text_bundle, FloatyText};
use crate::states::playing::hover_stats::HoverStats;
use crate::states::playing::init::MouseHoverText;
use crate::states::playing::spawn_entities::SpawnEntityEvent;
use crate::BillboardMaterial;
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use naia_bevy_client::Client;
use shared::game::defs::{Creep, CreepRef, Defs, Tower, TowerRef};
use shared::game::owner::Owner;
use shared::game::position::vec2_to_vec3;
use shared::game::shared_game::{ServerEntityId, SharedGame};
use shared::game::ClientGameInfo;
use shared::protocol::combo_creep_request::ComboCreepRequest;
use shared::protocol::combo_tower_request::ComboTowerRequest;
use shared::protocol::request_tower_placement::NewTowerRequest;
use shared::protocol::Protocol;
use shared::Channels;

#[derive(Component)]
pub struct Guide;

#[derive(Debug)]
pub enum HoveringOn {
    Nothing,
    Creep(ServerEntityId, Vec3, Creep),
    Tower(ServerEntityId, Vec3, Tower),
}

/// If this component does not exist, nothing is selected.
#[derive(Component, Debug)]
pub enum Selected {
    Nothing,

    OneCreepForCombo {
        first_id: ServerEntityId,
        position: Vec3,
        creep: Creep,
    },

    /// One tower is clicked on and waiting for a second tower.
    OneTowerForCombo {
        first_id: ServerEntityId,
        position: Vec3,
        tower: Tower,
    },
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SetGuidePosition {
    Normal,
    Lock(Vec3),
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum SetGuideVisibility {
    Off,
    Good,
    Bad,
}

#[derive(Debug)]
struct SetGuide {
    visibility: SetGuideVisibility,
    position: SetGuidePosition,
}

impl SetGuide {
    fn new() -> Self {
        Self {
            visibility: SetGuideVisibility::Off,
            position: SetGuidePosition::Normal,
        }
    }
}

#[derive(Component)]
pub struct CombineFloatyText;

pub fn mouse_action(
    mut commands: Commands,
    client_game_info: Query<&ClientGameInfo>,
    defs: Res<Defs>,
    mut client: Client<Protocol, Channels>,
    buttons: Res<Input<MouseButton>>,
    query: Query<&Intersection<MyRaycastSet>, (Without<Guide>, Without<TowerRef>)>,
    towers: Query<(&TowerRef, &Transform, &ServerEntityId, &Owner), Without<Guide>>,
    creeps: Query<(&CreepRef, &Transform, &ServerEntityId, &Owner, &Released), (Without<Guide>)>,
    mut materials: ResMut<Assets<BillboardMaterial>>,
    mut guide: Query<
        (&mut Transform, &Handle<BillboardMaterial>),
        (With<Guide>, Without<TowerRef>),
    >,
    mut selected: ResMut<Selected>,
    mut hover_text_query: Query<
        &mut FloatyText,
        (With<MouseHoverText>, Without<CombineFloatyText>),
    >,
    mut combine_floaty_text_query: Query<Entity, (With<FloatyText>, With<CombineFloatyText>)>,
) {
    let client_game_info = match client_game_info.get_single() {
        Ok(client_game_info) => client_game_info,
        Err(_) => {
            warn!("No client_game_info");
            return;
        }
    };

    let level = match defs.levels.get(&client_game_info.map) {
        None => {
            warn!("No level");
            return;
        }
        Some(level) => level,
    };

    let (mut guide_transform, material_handle) = if let Ok(g) = guide.get_single_mut() {
        g
    } else {
        // Haven't set up the guide yet!
        warn!("No guide");
        return;
    };

    let mut position = None;
    for intersection in query.iter() {
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

    let base_tower = if let Some(tower) = defs.tower(&TowerRef("machine".to_string())) {
        tower
    } else {
        return;
    };

    let mut hovering_on = HoveringOn::Nothing;
    for (other_tower_ref, transform, server_entity_id, tower_owner) in towers.iter() {
        if client_game_info.i_am != *tower_owner {
            continue;
        }
        let other_tower = defs.tower(&other_tower_ref).unwrap();
        // let min_distance = &other_tower.size / 2.0 + &other_tower.size / 2.0;
        let min_distance = 4.0;
        let distance = (transform.translation - mouse_position_vec3).length();
        if distance < min_distance {
            hovering_on = HoveringOn::Tower(
                *server_entity_id,
                transform.translation,
                other_tower.clone(),
            );
        }
    }

    let mut closest_creep = None;
    for (other_creep_ref, transform, server_entity_id, creep_owner, released) in creeps.iter() {
        // Can't upgrade creeps after they've released.
        if released.0 == true {
            continue;
        }
        if client_game_info.i_am != *creep_owner {
            continue;
        }
        let creep = defs.creep(&other_creep_ref).unwrap();
        let max_distance = creep.size / 2.0;
        let distance = (transform.translation - mouse_position_vec3).length();
        if distance > max_distance {
            continue;
        }
        match closest_creep {
            None => {
                closest_creep = Some(distance);
                hovering_on = HoveringOn::Creep(*server_entity_id, transform.translation, creep);
            }
            Some(previous_closest) => {
                if distance < previous_closest {
                    closest_creep = Some(distance);
                    hovering_on =
                        HoveringOn::Creep(*server_entity_id, transform.translation, creep);
                }
            }
        }
    }

    let mut set_guide = SetGuide::new();
    let mut set_text: String = "".to_string();
    match &*selected {
        Selected::Nothing => {
            // Nothing selected
            match hovering_on {
                HoveringOn::Nothing => {
                    // Work out if the player can build here first.

                    if level.can_build_here(client_game_info.i_am, &mouse_position_vec2) {
                        set_text = "Place a tower here".into();
                        set_guide = SetGuide {
                            visibility: SetGuideVisibility::Good,
                            position: SetGuidePosition::Normal,
                        };

                        if buttons.just_released(MouseButton::Left) {
                            let place_tower =
                                NewTowerRequest::new(mouse_position_vec2, "machine", 1230);
                            client.send_message(Channels::PlayerCommand, &place_tower);
                        }
                    } else {
                        set_text = "Try building somewhere else!".into();
                        set_guide = SetGuide {
                            visibility: SetGuideVisibility::Bad,
                            position: SetGuidePosition::Normal,
                        };
                    }
                }
                HoveringOn::Creep(hovering_creep_id, hovering_creep_position, creep) => {
                    set_text = format!("Combine this {}\nwith another creep.", creep.title);
                    set_guide = SetGuide {
                        visibility: SetGuideVisibility::Good,
                        position: SetGuidePosition::Lock(hovering_creep_position),
                    };

                    if buttons.just_released(MouseButton::Left) {
                        *selected = Selected::OneCreepForCombo {
                            first_id: hovering_creep_id,
                            position: hovering_creep_position,
                            creep,
                        };
                    }
                }
                HoveringOn::Tower(hovering_tower_id, hovering_tower_position, tower) => {
                    set_text = format!("Combine this {}\nwith another tower.", tower.title);
                    set_guide = SetGuide {
                        visibility: SetGuideVisibility::Good,
                        position: SetGuidePosition::Lock(hovering_tower_position),
                    };

                    if buttons.just_released(MouseButton::Left) {
                        *selected = Selected::OneTowerForCombo {
                            first_id: hovering_tower_id,
                            position: hovering_tower_position,
                            tower,
                        };
                    }
                }
            }
        }

        Selected::OneTowerForCombo {
            first_id: first_tower_id,
            position: first_tower_position,
            tower: first_tower,
        } => {
            // Already have one tower selected.
            let mut find_another = false;
            match hovering_on {
                HoveringOn::Tower(hovering_tower_id, hovering_tower_pos, hovering_tower) => {
                    // Hovering on the second tower.

                    if first_tower_id == &hovering_tower_id {
                        find_another = true;
                    } else {
                        match defs.tower_for_combo(&vec![&hovering_tower.name, &first_tower.name]) {
                            Some(new_tower) => {
                                set_text = format!(
                                    "Click to combine\ninto a {}.\n${}",
                                    new_tower.title, new_tower.cost
                                );
                                set_guide = SetGuide {
                                    visibility: SetGuideVisibility::Good,
                                    position: SetGuidePosition::Lock(hovering_tower_pos),
                                };

                                if buttons.just_released(MouseButton::Left) {
                                    let combo_tower = ComboTowerRequest::new(vec![
                                        *first_tower_id,
                                        hovering_tower_id,
                                    ]);
                                    client.send_message(Channels::PlayerCommand, &combo_tower);

                                    *selected = Selected::Nothing;
                                    combine_floaty_text_query.for_each(|e| {
                                        commands.entity(e).despawn();
                                    });
                                }
                            }
                            None => {
                                set_text = format!(
                                    "Can't combine {}\n with a {}.",
                                    first_tower.title, hovering_tower.title
                                );
                                set_guide = SetGuide {
                                    visibility: SetGuideVisibility::Bad,
                                    position: SetGuidePosition::Lock(hovering_tower_pos),
                                };
                            }
                        }
                    }
                }
                HoveringOn::Nothing => {
                    find_another = true;
                }
                HoveringOn::Creep(..) => {
                    find_another = true;
                }
            }

            if find_another {
                // Linking to nothing.
                set_text = "Find another tower to combine.\nClick here to abort!".to_string();
                set_guide = SetGuide {
                    visibility: SetGuideVisibility::Bad,
                    position: SetGuidePosition::Normal,
                };

                if buttons.just_released(MouseButton::Left) {
                    *selected = Selected::Nothing;
                    combine_floaty_text_query.for_each(|e| {
                        commands.entity(e).despawn();
                    });
                }
            }
        }
        Selected::OneCreepForCombo {
            first_id: first_creep_id,
            position: first_creep_position,
            creep: first_creep,
        } => {
            let mut find_another = false;
            match hovering_on {
                HoveringOn::Creep(hovering_creep_id, hovering_creep_pos, hovering_creep) => {
                    // Hovering on the second creep.
                    if first_creep_id == &hovering_creep_id {
                        find_another = true;
                    } else {
                        match defs.creep_for_combo(&vec![&hovering_creep.name, &first_creep.name]) {
                            Some(new_creep) => {
                                set_text = format!(
                                    "Click to combine\ninto a {}.\n${}",
                                    new_creep.title, new_creep.cost
                                );
                                set_guide = SetGuide {
                                    visibility: SetGuideVisibility::Good,
                                    position: SetGuidePosition::Lock(hovering_creep_pos),
                                };

                                if buttons.just_released(MouseButton::Left) {
                                    let combo_creep = ComboCreepRequest::new(vec![
                                        *first_creep_id,
                                        hovering_creep_id,
                                    ]);
                                    client.send_message(Channels::PlayerCommand, &combo_creep);

                                    *selected = Selected::Nothing;
                                    combine_floaty_text_query.for_each(|e| {
                                        commands.entity(e).despawn();
                                    });
                                }
                            }
                            None => {
                                set_text = format!(
                                    "Can't combine a {}\n with a {}.",
                                    first_creep.title, hovering_creep.title
                                );
                                set_guide = SetGuide {
                                    visibility: SetGuideVisibility::Bad,
                                    position: SetGuidePosition::Lock(hovering_creep_pos),
                                };
                            }
                        }
                    }
                }
                HoveringOn::Nothing => {
                    find_another = true;
                }
                HoveringOn::Tower(..) => find_another = true,
            }

            if find_another {
                // Linking to nothing.
                set_text = "Find another creep to combine.\nClick here to abort!".to_string();
                set_guide = SetGuide {
                    visibility: SetGuideVisibility::Bad,
                    position: SetGuidePosition::Normal,
                };

                if buttons.just_released(MouseButton::Left) {
                    *selected = Selected::Nothing;
                    combine_floaty_text_query.for_each(|e| {
                        commands.entity(e).despawn();
                    });
                }
            }
        }
    }

    let mut material = materials.get_mut(&material_handle).unwrap();
    match set_guide.visibility {
        SetGuideVisibility::Off => {
            material.color = Color::rgba(0.0, 0.0, 0.0, 0.0);
        }
        SetGuideVisibility::Good => {
            material.color = Color::rgba(0.0, 1.0, 0.0, 0.2);
        }
        SetGuideVisibility::Bad => {
            material.color = Color::rgba(1.0, 0.0, 0.0, 0.2);
        }
    }

    match set_guide.position {
        SetGuidePosition::Normal => {
            guide_transform.translation = vec2_to_vec3(&mouse_position_vec2);
        }
        SetGuidePosition::Lock(pos) => {
            guide_transform.translation = pos;
        }
    }
    guide_transform.translation += Vec3::new(0.0, 0.5, 0.0);

    let mut floaty = hover_text_query.single_mut();
    floaty.world_position = guide_transform.translation;
    floaty.text = set_text;
}
