use crate::app::MyRaycastSet;
use crate::states::playing::spawn_entities::SpawnEntityEvent;
use crate::BillboardMaterial;
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use naia_bevy_client::Client;
use shared::game::defs::{Defs, TowerRef};
use shared::game::position::vec2_to_vec3;
use shared::game::shared_game::{CanBuild, SharedGame};
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

pub fn left_click(
    defs: Res<Defs>,
    mut client: Client<Protocol, Channels>,
    buttons: Res<Input<MouseButton>>,
    mut spawn_entities: EventWriter<SpawnEntityEvent>,
    query: Query<&Intersection<MyRaycastSet>, (Without<Guide>, Without<TowerRef>)>,
    towers: Query<(&TowerRef, &Transform), Without<Guide>>,
    mut materials: ResMut<Assets<BillboardMaterial>>,
    mut guide: Query<
        (&mut Transform, &Handle<BillboardMaterial>),
        (With<Guide>, Without<TowerRef>),
    >,
) {
    let (mut guide_transform, material_handle) = if let Ok(g) = guide.get_single_mut() {
        g
    } else {
        // Haven't set up the guide yet!
        warn!("no guide");
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

    let tower_size_squared = tower.size * tower.size;
    let mut can_build = CanBuild::Yes;
    for (other_tower_ref, transform) in towers.iter() {
        let other_tower = defs.tower(&other_tower_ref.0).unwrap();
        let distance_squared = (transform.translation - vec2_to_vec3(&position)).length_squared();
        if distance_squared < tower_size_squared {
            can_build = CanBuild::No("Build further from other towers!".into());
            break;
        }
    }

    let mut material = materials.get_mut(&material_handle).unwrap();
    if let CanBuild::Yes = can_build {
        material.color = Color::rgba(0.0, 1.0, 0.0, 0.1);
    } else {
        material.color = Color::rgba(1.0, 0.0, 0.0, 0.1);
    }

    guide_transform.translation = vec2_to_vec3(&position) + Vec3::new(0.0, 0.5, 0.0);

    if !(buttons.just_released(MouseButton::Left)) {
        return;
    }

    let place_tower = RequestTowerPlacement::new(position, "machine", 1230);
    client.send_message(Channels::PlayerCommand, &place_tower);

    println!("sent place tower request");
}
