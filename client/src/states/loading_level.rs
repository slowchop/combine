use crate::app::GameState;
use crate::states::playing::spawn_entities::SpawnEntityEvent;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use shared::game::defs::{Defs, LevelDef};
use shared::game::shared_game::SharedGame;
use shared::game::ClientGameInfo;

pub fn init(
    mut commands: Commands,
    game_info: Query<&ClientGameInfo>,
    mut new_entities: EventWriter<SpawnEntityEvent>,
    defs: Res<Defs>,
) {
    let game_info = game_info.single();
    println!("Loading level: {}", game_info.map);

    let level = &defs.levels[&game_info.map];

    new_entities.send_batch(level.entities.iter().map(|e| SpawnEntityEvent {
        server_entity_id: None,
        entity_def: e.clone(),
        speed_multiplier: 1.0,
        health_multiplier: 1.0,
    }));

    commands.insert_resource(NextState(GameState::Playing));
    println!("Loading level done!");
}
