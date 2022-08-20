use crate::app::GameState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub fn init(mut commands: Commands, time: Res<Time>) {
    println!("Loading...");
}

pub fn update(mut commands: Commands, time: Res<Time>) {
    println!("Waiting for loading...");
    commands.insert_resource(NextState(GameState::MainMenu));
}
