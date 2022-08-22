use crate::app::MyRaycastSet;
use crate::states::playing::spawn_entities::SpawnEntity;
use crate::states::playing::GameInfo;
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use naia_bevy_client::Client;
use shared::game::managed_game::{CanBuild, ManagedGame};
use shared::game::towers::Tower;
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

pub fn left_click(
    mut commands: Commands,
    mut client: Client<Protocol, Channels>,
    buttons: Res<Input<MouseButton>>,
    query: Query<&Intersection<MyRaycastSet>>,
    mut spawn_entities: EventWriter<SpawnEntity>,
    game: Query<&mut ManagedGame>,
    game_info: Query<&GameInfo>,
) {
    if !(buttons.just_released(MouseButton::Left)) {
        return;
    }

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

    let game = game.single();
    let game_info = game_info.single();
    if let CanBuild::No(reason) =
        game.can_build_tower(&game_info.i_am, &position, &Tower::MachineGun)
    {
        info!("Can't build! {}", reason);
        return;
    }

    let place_tower = RequestTowerPlacement::new(position, Tower::MachineGun, 1230);
    client.send_message(Channels::PlayerCommand, &place_tower);

    println!("sent place tower request");
}
