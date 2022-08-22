use crate::app::{GameState, MyRaycastSet};
use crate::states::playing::bottom_quad::BottomQuad;
use crate::states::playing::spawn_entities::SpawnEntity;
use crate::states::playing::GameInfo;
use crate::{BillboardMaterial, Textures, YamlLevel};
use bevy::asset::LoadState;
use bevy::ecs::system::EntityCommands;
use bevy::ecs::world::EntityMut;
use bevy::prelude::*;
use bevy_mod_raycast::RayCastMesh;
use iyes_loopless::prelude::*;
use shared::game::level::PIXELS_PER_METER;
use shared::game::managed_game::ManagedGame;
use std::f32::consts::TAU;

pub fn init(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    game_info_query: Query<&GameInfo>,
) {
    let game_info = game_info_query.single();

    let level_path = format!("levels/{}.level", game_info.level);
    println!("Loading level... {}", level_path);

    commands.insert_resource(asset_server.load::<Textures, _>("game.textures"));
    commands.insert_resource(asset_server.load::<YamlLevel, _>(&level_path));
}

pub fn spawn_level(
    mut commands: Commands,
    game_info: Query<&GameInfo>,
    level: Res<Handle<YamlLevel>>,
    level_assets: ResMut<Assets<YamlLevel>>,
    textures: Res<Handle<Textures>>,
    textures_assets: ResMut<Assets<Textures>>,
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
    let level: &YamlLevel = level_assets.get(&level).unwrap();
    let textures: &Textures = textures_assets.get(&textures).unwrap();

    let game_info = game_info.single();

    let managed_game = ManagedGame::from_players_level_textures(
        game_info.players.clone(),
        level.entities.as_slice(),
        textures.0.as_slice(),
    );
    commands.spawn().insert(managed_game);

    new_entities.send_batch(level.entities.iter().map(|e| SpawnEntity(e.clone())));

    commands.insert_resource(NextState(GameState::Playing));
    println!("Loading level done!");
}
