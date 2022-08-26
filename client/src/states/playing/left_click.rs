use crate::app::MyRaycastSet;
use crate::states::playing::creeps::Released;
use crate::states::playing::floaty_text::{floaty_text_bundle, FloatyText};
use crate::states::playing::init::MouseHoverText;
use crate::states::playing::spawn_entities::SpawnEntityEvent;
use crate::BillboardMaterial;
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use naia_bevy_client::Client;
use shared::game::defs::{CreepRef, Defs, Tower, TowerRef};
use shared::game::owner::Owner;
use shared::game::position::vec2_to_vec3;
use shared::game::shared_game::{ServerEntityId, SharedGame};
use shared::game::ClientGameInfo;
use shared::protocol::combo_tower_request::ComboTowerRequest;
use shared::protocol::request_tower_placement::NewTowerRequest;
use shared::protocol::Protocol;
use shared::Channels;

#[derive(Component)]
pub struct Guide;

#[derive(Debug)]
pub enum HoveringOn {
    Nothing,
    Creep(ServerEntityId, Vec3),
    Tower(ServerEntityId, Vec3),
}

/// If this component does not exist, nothing is selected.
#[derive(Component, Debug)]
pub enum Selected {
    Nothing,

    /// One tower is clicked on and waiting for a second tower.
    OneTowerForCombo {
        first_id: ServerEntityId,
        position: Vec3,
    },

    /// A second tower is selected for a combo.
    ///
    /// This should show a confirmation thing.
    TwoTowersForCombo {
        first: ServerEntityId,
        second: ServerEntityId,
        to_build: TowerRef,
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
    asset_server: Res<AssetServer>,
    defs: Res<Defs>,
    mut client: Client<Protocol, Channels>,
    buttons: Res<Input<MouseButton>>,
    query: Query<&Intersection<MyRaycastSet>, (Without<Guide>, Without<TowerRef>)>,
    towers: Query<(&TowerRef, &Transform, &ServerEntityId, &Owner), Without<Guide>>,
    creeps: Query<
        (&CreepRef, &Transform, &ServerEntityId, &Owner),
        (Without<Guide>, Without<Released>),
    >,
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

    let (mut guide_transform, material_handle) = if let Ok(g) = guide.get_single_mut() {
        g
    } else {
        // Haven't set up the guide yet!
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

    let tower = if let Some(tower) = defs.tower("machine") {
        tower
    } else {
        return;
    };

    let mut hovering_on = HoveringOn::Nothing;
    for (other_tower_ref, transform, server_entity_id, tower_owner) in towers.iter() {
        if client_game_info.i_am != *tower_owner {
            continue;
        }
        let other_tower = defs.tower(&other_tower_ref.0).unwrap();
        let min_distance = other_tower.size / 2.0 + tower.size / 2.0;
        let distance = (transform.translation - mouse_position_vec3).length();
        if distance < min_distance {
            hovering_on = HoveringOn::Tower(*server_entity_id, transform.translation);
        }
    }

    let mut closest_creep = None;
    for (other_creep_ref, transform, server_entity_id, creep_owner) in creeps.iter() {
        if client_game_info.i_am != *creep_owner {
            continue;
        }
        let creep = defs.creep(&other_creep_ref.0).unwrap();
        let max_distance = creep.size / 2.0;
        let distance = (transform.translation - mouse_position_vec3).length();
        if distance > max_distance {
            continue;
        }
        match closest_creep {
            None => {
                closest_creep = Some(distance);
                hovering_on = HoveringOn::Creep(*server_entity_id, transform.translation);
            }
            Some(previous_closest) => {
                if distance < previous_closest {
                    closest_creep = Some(distance);
                    hovering_on = HoveringOn::Creep(*server_entity_id, transform.translation);
                }
            }
        }
    }

    // println!("hovering on: {:?}", hovering_on);

    let mut set_guide = SetGuide::new();
    let mut set_text = None;
    match &*selected {
        Selected::Nothing => {
            // Nothing selected
            match hovering_on {
                HoveringOn::Nothing => {
                    set_text = Some("Place a tower here");
                    set_guide = SetGuide {
                        visibility: SetGuideVisibility::Good,
                        position: SetGuidePosition::Normal,
                    };

                    if buttons.just_released(MouseButton::Left) {
                        let place_tower =
                            NewTowerRequest::new(mouse_position_vec2, "machine", 1230);
                        client.send_message(Channels::PlayerCommand, &place_tower);
                    }
                }
                HoveringOn::Creep(hovering_creep_id, hovering_creep_position) => {
                    todo!()
                }
                HoveringOn::Tower(hovering_tower_id, hovering_tower_position) => {
                    set_text = Some("Combine this tower\nwith another.");
                    set_guide = SetGuide {
                        visibility: SetGuideVisibility::Good,
                        position: SetGuidePosition::Lock(hovering_tower_position),
                    };

                    if buttons.just_released(MouseButton::Left) {
                        *selected = Selected::OneTowerForCombo {
                            first_id: hovering_tower_id,
                            position: hovering_tower_position,
                        };
                        // TODO: Doesn't work
                        // commands
                        //     .spawn_bundle(floaty_text_bundle(&asset_server))
                        //     .insert(FloatyText {
                        //         text: "#1".to_string(),
                        //         world_position: hovering_tower_position,
                        //     })
                        //     .insert(CombineFloatyText);
                    }
                }
            }
        }

        Selected::OneTowerForCombo {
            first_id: first_tower_id,
            position: first_tower_position,
        } => {
            // Already have one tower selected.
            let mut find_another = false;
            if let HoveringOn::Tower(hovering_tower_id, hovering_tower_pos) = hovering_on {
                // Hovering on the second tower.
                // TODO: Work out if the combo is OK

                if first_tower_id == &hovering_tower_id {
                    find_another = true;
                } else {
                    set_text = "Click to combine\ninto a ~RFlame Tower~N.\n$200".into();
                    set_guide = SetGuide {
                        visibility: SetGuideVisibility::Good,
                        position: SetGuidePosition::Lock(hovering_tower_pos),
                    };

                    if buttons.just_released(MouseButton::Left) {
                        let combo_tower =
                            ComboTowerRequest::new(vec![*first_tower_id, hovering_tower_id]);
                        client.send_message(Channels::PlayerCommand, &combo_tower);

                        *selected = Selected::Nothing;
                        combine_floaty_text_query.for_each(|e| {
                            commands.entity(e).despawn();
                        });
                    }
                }
            } else {
                find_another = true;
            }

            if find_another {
                // Linking to nothing.
                set_text = "Find another tower to combine.\nClick here to abort!".into();
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
        Selected::TwoTowersForCombo {
            first,
            second,
            to_build,
        } => todo!(),
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
    floaty.text = set_text.unwrap_or("").to_string();
}
