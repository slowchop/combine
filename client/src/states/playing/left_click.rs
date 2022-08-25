use crate::app::MyRaycastSet;
use crate::states::playing::spawn_entities::SpawnEntityEvent;
use crate::BillboardMaterial;
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use naia_bevy_client::Client;
use shared::game::defs::{Defs, Tower, TowerRef};
use shared::game::position::vec2_to_vec3;
use shared::game::shared_game::{CanBuild, ServerEntityId, SharedGame};
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
enum SetGuide {
    Off,
    Normal,
    Lock(Vec3),
}

pub enum CanBuild {
    BaseTower,
    CombinedTower {
        tower_1: ServerEntityId,
        tower_2: ServerEntityId,
        tower_ref: TowerRef,
    },
    No,
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
    mut selected: Query<&mut Selected>,
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

    let mut set_guide = SetGuide::Off;
    let mut set_text = None;
    let mut can_build = CanBuild::Yes;
    match selected.get_single_mut().as_deref_mut() {
        Err(_) => {
            println!("nothing selected");
            if let HoveringOn::Tower(next_tower_id, next_tower_pos) = hovering_on {
                set_guide = SetGuide::Lock(next_tower_pos);
                can_build = CanBuild::No;
            } else {
                set_guide = SetGuide::Normal;
                can_build = CanBuild::BaseTower;
                set_text = Some("Place a tower here");
            }
        }
        Ok(Selected::OneTowerForCombo(tower_1_id)) => {
            println!("one tower for combo");
            if let HoveringOn::Tower(next_tower_id, next_tower_pos) = hovering_on {
                if tower_1_id == &next_tower_id {
                    set_guide = SetGuide::Off;
                } else {
                    // TODO: Work out if the combo is OK
                    set_guide = SetGuide::Lock(next_tower_pos);
                }
            }
        }
        _ => todo!(),
    }

    let mut material = materials.get_mut(&material_handle).unwrap();
    if let CanBuild::Yes = can_build {
        material.color = Color::rgba(0.0, 1.0, 0.0, 0.1);
    } else {
        material.color = Color::rgba(1.0, 0.0, 0.0, 0.1);
    }

    if set_guide == SetGuide::Off {
        material.color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    } else if let SetGuide::Lock(pos) = set_guide {
        guide_transform.translation = pos;
    } else if let SetGuide::Normal = set_guide {
        guide_transform.translation = vec2_to_vec3(&position);
    }
    guide_transform.translation += Vec3::new(0.0, 0.5, 0.0);

    if !(buttons.just_released(MouseButton::Left)) {
        return;
    }

    let place_tower = RequestTowerPlacement::new(position, "machine", 1230);
    client.send_message(Channels::PlayerCommand, &place_tower);

    println!("sent place tower request");
}
