pub mod bottom_quad;
pub mod camera;
pub mod left_click;
pub mod spawn_entities;
pub mod time;
pub mod ui;

use bevy::prelude::*;
use shared::game::owner::Owner;
use shared::game::player::PlayerName;
use shared::game::player::SharedPlayer;
use shared::protocol::game_ready::GameReady;
