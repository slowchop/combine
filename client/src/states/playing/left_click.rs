use crate::app::MyRaycastSet;
use crate::states::playing::spawn_entities::SpawnEntityEvent;
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
    mut guide: Query<&mut Transform, (With<Guide>, Without<TowerRef>)>,
) {
    let mut guide_transform = if let Ok(g) = guide.get_single_mut() {
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
            println!("no interection");
            continue;
        };
        position = Some(Vec2::new(intersection.x, intersection.z));
    }

    let position = if let Some(position) = position {
        position
    } else {
        println!("no raycast");
        return;
    };

    // for (tower_ref, transform) in towers.iter() {
    //     let tower = defs.tower(&tower_ref.0).unwrap();
    //     let distance_squared = (transform.translation - vec2_to_vec3(&position)).length_squared();
    // }

    guide_transform.translation = vec2_to_vec3(&position) + Vec3::new(0.0, 0.5, 0.0);

    if !(buttons.just_released(MouseButton::Left)) {
        return;
    }

    let place_tower = RequestTowerPlacement::new(position, "machine", 1230);
    client.send_message(Channels::PlayerCommand, &place_tower);

    println!("sent place tower request");
}
