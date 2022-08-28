pub mod bottom_quad;
pub mod camera;
pub mod console;
pub mod creeps;
pub mod debug_lines;
pub mod destroy_entities;
pub mod effect_bubble;
pub mod floaty_text;
pub mod game_over;
pub mod health_bars;
pub mod hover_stats;
pub mod hurt_entities;
pub mod init;
pub mod left_click;
pub mod projectiles;
pub mod spawn_entities;
pub mod time;
pub mod tooltips;
pub mod top_helper_text;
pub mod ui;
pub mod update_player;
pub mod update_positions;

use bevy::prelude::*;
use shared::game::owner::Owner;
use shared::game::player::PlayerName;
use shared::game::player::SharedPlayer;
use shared::protocol::game_ready::GameReady;
