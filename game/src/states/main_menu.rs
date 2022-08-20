use crate::app::GameState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

pub fn init(mut commands: Commands, time: Res<Time>) {
    println!("Main menu...");
}

pub fn update(mut commands: Commands, time: Res<Time>) {
    println!("Waiting for mainmenu...");
    if time.seconds_since_startup() > 2.5 {
        commands.insert_resource(NextState(GameState::Playing));
    }
}
