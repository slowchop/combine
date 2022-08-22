#![allow(clippy::too_many_arguments)]

mod app;
mod net;
mod other_systems;
mod settings;
mod shader;
mod states;
mod textures;

use crate::shader::BillboardMaterial;
use crate::textures::update_texture_sizes;
use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use clap::Parser;
use shared::game::level::{LevelLoadState, Textures, YamlLevel};
use states::playing::camera::move_camera;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    command: Option<Command>,

    #[clap(short)]
    skip_to_random_player: bool,

    /// Debugging hack to run two clients. False 0, True 1.
    #[clap(short)]
    window_position_shift: Option<u8>,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    UpdateTextureSizes,
}

fn main() -> miette::Result<()> {
    let args = Args::parse();
    match args.command {
        None => app::play(&args),
        Some(Command::UpdateTextureSizes) => update_texture_sizes()?,
    }
    Ok(())
}
