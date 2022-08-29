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

    /// Sets the server to gak's IP address.
    #[clap(short)]
    debug: bool,

    /// Windowed mode
    #[clap(long)]
    windowed: bool,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Textures,
}

fn main() -> miette::Result<()> {
    #[cfg(not(target_arch = "wasm32"))]
    let _guard = sentry::init((
        "https://682d2e74603f4cc185e4b408f89f0e73@o1376616.ingest.sentry.io/6685785",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    let args = Args::parse();
    match args.command {
        None => app::play(&args),
        Some(Command::Textures) => update_texture_sizes()?,
    }
    Ok(())
}
