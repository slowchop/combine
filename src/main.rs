mod level;
mod bottom_quad;
mod shader;
mod textures;
mod camera;
mod app;
mod other_systems;
mod settings;

use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;
use crate::level::{Level, LevelLoadState, spawn_level, TextureDefinition, Textures};
use crate::textures::update_texture_sizes;
use clap::Parser;
use crate::shader::BillboardMaterial;
use bevy_inspector_egui::WorldInspectorPlugin;
use crate::camera::move_camera;

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
