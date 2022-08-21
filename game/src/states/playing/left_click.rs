use crate::app::MyRaycastSet;
use crate::states::playing::spawn_entities::SpawnEntity;
use crate::states::playing::GameInfo;
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use naia_bevy_client::Client;
use shared::game::managed_game::{CanBuild, EntityType, LevelEntity, ManagedGame};
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
    dbg!(position);

    let game = game.single();
    let game_info = game_info.single();
    if let CanBuild::No(reason) =
        game.can_build_tower(&game_info.you_are, &position, &Tower::MachineGun)
    {
        info!("Can't build! {}", reason);
        return;
    }

    if game_info.multiplayer {
        let mut place_tower = RequestTowerPlacement::new(position, Tower::MachineGun, 1230);
        client.send_message(Channels::PlayerCommand, &mut place_tower);
    } else {
        spawn_entities.send(SpawnEntity(LevelEntity {
            texture: "harold.png".to_string(),
            entity_type: EntityType::Sprite,
            position,
            owner: Some(game_info.you_are),
            radius: None,
        }));
    }

    //     spawn_entities.send(SpawnEntity(LevelEntity {
    //         entity_type: EntityType::Tower,
    //         position,
    //     }));
    //     client.send(RequestTowerPlacement {
    //         tower: Tower::new(position),
    //     });
    // }
    //
    // let mut place_tower = RequestTowerPlacement::new(position, Tower::MachineGun, 1230);
    // client.send_message(Channels::PlayerCommand, &mut place_tower);
    //
    // let level_entity = LevelEntity {
    //     texture: "harold.png".to_string(),
    //     position: position.into(),
    //     test: Default::default(),
    //     entity_type: EntityType::Sprite,
    //     owner: None,
    // };
    // spawn_entities.send(SpawnEntity(level_entity));
}
