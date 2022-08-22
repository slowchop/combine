use crate::app::GameState;
use crate::states::playing::bottom_quad::BottomQuad;
use crate::{BillboardMaterial, Textures, YamlLevel};
use bevy::asset::LoadState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use shared::game::level::PIXELS_PER_METER;
use std::f32::consts::TAU;

pub fn init(mut commands: Commands, time: Res<Time>) {
    println!("Loading...");
}

pub fn update(mut commands: Commands, time: Res<Time>) {
    commands.insert_resource(asset_server.load::<Defs, _>("game.defs"));
    
    println!("Waiting for loading...");
    commands.insert_resource(NextState(GameState::MainMenu));
}
