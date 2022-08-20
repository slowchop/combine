use crate::app::GameState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub fn init(mut commands: Commands, time: Res<Time>) {
    println!("Loading...");
}

pub fn update(mut commands: Commands, time: Res<Time>) {
    println!("Waiting for loading...");
    if time.seconds_since_startup() > 2.5 {
        commands.insert_resource(NextState(GameState::MainMenu));
    }
}
