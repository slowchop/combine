mod app;
mod bottom_quad;
mod camera;
mod level;
mod net;
mod other_systems;
mod settings;
mod shader;
mod textures;

use crate::camera::move_camera;
use crate::level::{spawn_level, Level, LevelLoadState, Textures};
use crate::shader::BillboardMaterial;
use crate::textures::update_texture_sizes;
use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    UpdateTextureSizes,
}

fn main() -> miette::Result<()> {
    let args = Args::parse();
    match args.command {
        None => app::play(),
        Some(Command::UpdateTextureSizes) => update_texture_sizes()?,
    }
    Ok(())
}
