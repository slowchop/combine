use crate::app::MyRaycastSet;
use crate::states::playing::init::HoverText;
use crate::states::playing::spawn_entities::SpawnEntityEvent;
use crate::BillboardMaterial;
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use naia_bevy_client::Client;
use shared::game::defs::{Defs, Tower, TowerRef};
use shared::game::position::vec2_to_vec3;
use shared::game::shared_game::{ServerEntityId, SharedGame};
use shared::game::ClientGameInfo;
use shared::protocol::request_tower_placement::RequestTowerPlacement;
use shared::protocol::Protocol;
use shared::Channels;

// pub fn place_tower_requests(
//     mut request_tower_placement: EventReader<RequestTowerPlacement>,
//     game_info: Query<&GameInfo>,
// ) {
//     let game_info = game_info.single();
//     for request in request_tower_placement.iter() {}
// }

#[derive(Component)]
pub struct Guide;

#[derive(Debug)]
pub enum HoveringOn {
    Nothing,
    Tower(ServerEntityId, Vec3),
}

/// If this component does not exist, nothing is selected.
#[derive(Component, Debug)]
pub enum Selected {
    Nothing,

    /// One tower is clicked on and waiting for a second tower.
    OneTowerForCombo(ServerEntityId),

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
enum OnClick {
    Nothing,
    SetSelected(Selected),
    BuildBaseTower,
    BuildCombinedTower {
        tower_1: ServerEntityId,
        tower_2: ServerEntityId,
        tower_ref: TowerRef,
    },
}

#[derive(Debug)]
struct SetGuide {
    visibility: SetGuideVisibility,
    position: SetGuidePosition,
    on_click: OnClick,
}

impl SetGuide {
    fn new() -> Self {
        Self {
            visibility: SetGuideVisibility::Off,
            position: SetGuidePosition::Normal,
            on_click: OnClick::Nothing,
        }
    }
}

pub fn mouse_action(
    defs: Res<Defs>,
    mut client: Client<Protocol, Channels>,
    buttons: Res<Input<MouseButton>>,
    mut spawn_entities: EventWriter<SpawnEntityEvent>,
    query: Query<&Intersection<MyRaycastSet>, (Without<Guide>, Without<TowerRef>)>,
    towers: Query<(&TowerRef, &Transform, &ServerEntityId), Without<Guide>>,
    mut materials: ResMut<Assets<BillboardMaterial>>,
    mut guide: Query<
        (&mut Transform, &Handle<BillboardMaterial>),
        (With<Guide>, Without<TowerRef>),
    >,
    mut selected: ResMut<Selected>,
    camera_query: Query<(&GlobalTransform, &Camera)>,
    mut hover_text_query: Query<(&mut Style, &mut Text), (With<HoverText>)>,
) {
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
    let position = if let Some(position) = position {
        position
    } else {
        return;
    };

    let tower = if let Some(tower) = defs.tower("machine") {
        tower
    } else {
        return;
    };

    let mut hovering_on = HoveringOn::Nothing;
    for (other_tower_ref, transform, server_entity_id) in towers.iter() {
        let other_tower = defs.tower(&other_tower_ref.0).unwrap();
        let min_distance = (other_tower.size + tower.size).powf(2.0);
        let distance_squared = (transform.translation - vec2_to_vec3(&position)).length_squared();
        if distance_squared < min_distance {
            hovering_on = HoveringOn::Tower(*server_entity_id, transform.translation);
            break;
        }
    }

    let mut set_guide = SetGuide::new();
    let mut set_text = None;
    match *selected {
        Selected::Nothing => {
            // Nothing selected
            if let HoveringOn::Tower(next_tower_id, next_tower_pos) = hovering_on {
                // We're hovering on the first tower, suggest to combo.
                set_text = Some("Combine this tower\nwith another.");
                set_guide = SetGuide {
                    visibility: SetGuideVisibility::Good,
                    position: SetGuidePosition::Lock(next_tower_pos),
                    on_click: OnClick::SetSelected(Selected::OneTowerForCombo(next_tower_id)),
                };
            } else {
                // Hovering on nothing.
                set_text = Some("Place a tower here");
                set_guide = SetGuide {
                    visibility: SetGuideVisibility::Good,
                    position: SetGuidePosition::Normal,
                    on_click: OnClick::BuildBaseTower,
                };
            }
        }

        Selected::OneTowerForCombo(tower_1_id) => {
            // Already have one tower selected.
            if let HoveringOn::Tower(next_tower_id, next_tower_pos) = hovering_on {
                if tower_1_id == next_tower_id {
                    // guide_position = SetGuidePosition::Off;
                } else {
                    // TODO: Work out if the combo is OK
                    // guide_position = SetGuidePosition::Lock(next_tower_pos);
                }
            }
        }
        _ => todo!(),
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
            guide_transform.translation = vec2_to_vec3(&position);
        }
        SetGuidePosition::Lock(pos) => {
            guide_transform.translation = pos;
        }
    }
    guide_transform.translation += Vec3::new(0.0, 0.5, 0.0);

    let (camera_transform, camera) = camera_query.single();
    let viewport_pos = camera
        .world_to_viewport(camera_transform, guide_transform.translation)
        .unwrap();
    let (mut hover_text_style, mut text) = hover_text_query.single_mut();
    hover_text_style.position.left = Val::Px(viewport_pos.x);
    hover_text_style.position.bottom = Val::Px(viewport_pos.y);
    text.sections[0].value = set_text.unwrap_or("").to_string();

    if !(buttons.just_released(MouseButton::Left)) {
        return;
    }

    match set_guide.on_click {
        OnClick::Nothing => {}
        OnClick::BuildBaseTower => {
            let place_tower = RequestTowerPlacement::new(position, "machine", 1230);
            client.send_message(Channels::PlayerCommand, &place_tower);
        }
        OnClick::SetSelected(s) => {
            todo!();
        }
        OnClick::BuildCombinedTower { .. } => {
            todo!()
        }
    }

    // let place_tower = RequestTowerPlacement::new(position, "machine", 1230);
    // client.send_message(Channels::PlayerCommand, &place_tower);

    println!("sent place tower request");
}
