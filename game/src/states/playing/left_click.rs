use crate::app::MyRaycastSet;
use crate::states::playing::GameInfo;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use naia_bevy_client::Client;
use shared::protocol::request_tower_placement::RequestTowerPlacement;
use shared::protocol::Protocol;
use shared::towers::Tower;
use shared::Channels;

pub fn left_click(
    mut commands: Commands,
    mut client: Client<Protocol, Channels>,
    buttons: Res<Input<MouseButton>>,
    query: Query<&Intersection<MyRaycastSet>>,
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
    dbg!(position);

    let mut place_tower = RequestTowerPlacement::new(position, Tower::MachineGun, 1230);
    client.send_message(Channels::PlayerCommand, &mut place_tower);

    // commands.spawn_bundle()
    todo!()
}
