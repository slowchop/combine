pub mod bottom_quad;
pub mod camera;
pub mod creeps;
pub mod destroy_entities;
pub mod game_over;
pub mod left_click;
pub mod spawn_entities;
pub mod time;
pub mod ui;
pub mod update_player;
pub mod update_positions;

use bevy::prelude::*;
use shared::game::owner::Owner;
use shared::game::player::PlayerName;
use shared::game::player::SharedPlayer;
use shared::protocol::game_ready::GameReady;
