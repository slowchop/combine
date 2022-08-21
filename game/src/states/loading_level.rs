use crate::app::{GameState, MyRaycastSet};
use crate::states::playing::bottom_quad::BottomQuad;
use crate::states::playing::level::{EntityType, LevelEntity, PIXELS_PER_METER};
use crate::states::playing::GameInfo;
use crate::states::spawn_entities::SpawnEntity;
use crate::{BillboardMaterial, Level, Textures};
use bevy::asset::LoadState;
use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy_mod_raycast::RayCastMesh;
use iyes_loopless::prelude::*;
use std::f32::consts::TAU;

pub fn init(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    game_info_query: Query<&GameInfo>,
) {
    // let game_info = game_info_query.single();
    // let game_info = if let Ok(game_info) = game_info {
    //     game_info
    // } else {
    //     println!("Exiting loading_level::init early");
    //     return;
    // };
    let game_info = game_info_query.iter().next().unwrap();

    let level_path = format!("levels/{}.level", game_info.level);
    println!("Loading level... {}", level_path);

    commands.insert_resource(asset_server.load::<Textures, _>("game.textures"));
    commands.insert_resource(asset_server.load::<Level, _>(&level_path));
}

pub fn spawn_level(
    mut commands: Commands,
    level: Res<Handle<Level>>,
    level_assets: ResMut<Assets<Level>>,
    textures: Res<Handle<Textures>>,
    asset_server: Res<AssetServer>,
    mut new_entities: EventWriter<SpawnEntity>,
) {
    println!("Waiting for level info to load...");
    if asset_server.get_load_state(&*level) != LoadState::Loaded {
        return;
    }

    println!("Waiting for texture info to load...");
    if asset_server.get_load_state(&*textures) != LoadState::Loaded {
        return;
    }

    println!("Loading level...");
    let level: &Level = level_assets.get(&level).unwrap();
    new_entities.send_batch(level.entities.iter().map(|e| SpawnEntity(e.clone())));

    commands.insert_resource(NextState(GameState::Playing));
    println!("Loading level done!");
}
