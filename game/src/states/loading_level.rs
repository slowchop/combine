use crate::app::GameState;
use crate::states::playing::spawn_entities::SpawnEntityEvent;
use crate::states::playing::GameInfo;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use shared::game::defs::{Defs, LevelDef};
use shared::game::shared_game::SharedGame;

pub fn init(
    mut commands: Commands,
    game_info: Query<&GameInfo>,
    asset_server: Res<AssetServer>,
    mut new_entities: EventWriter<SpawnEntityEvent>,
    defs: Res<Defs>,
) {
    let game_info = game_info.single();
    println!("Loading level: {}", game_info.level);

    let level = &defs.levels[&game_info.level];

    // let textures: &Textures = textures_assets.get(&textures).unwrap();
    //
    // let game_info = game_info.single();
    //
    // let managed_game = ManagedGame::from_players_level_textures(
    //     game_info.players.clone(),
    //     level.entities.as_slice(),
    //     textures.0.as_slice(),
    // );
    // commands.spawn().insert(managed_game);
    warn!("TODO managed game in spawn_level");

    new_entities.send_batch(level.entities.iter().map(|e| SpawnEntityEvent(e.clone())));

    commands.insert_resource(NextState(GameState::Playing));
    println!("Loading level done!");
}
