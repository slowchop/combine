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
use shared::game::defs::{Defs, EntityType, NetVec2};
use shared::game::owner::Owner;
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

    /// Windowed mode
    #[clap(long)]
    map_editor: bool,
}

#[derive(Debug, clap::Subcommand)]
enum Command {
    Textures,
    MirrorHack,
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
        Some(Command::MirrorHack) => mirror_hack()?,
    }
    Ok(())
}

fn mirror_hack() -> miette::Result<()> {
    let mut defs = Defs::load();

    let map = defs.levels.get_mut("j").unwrap();
    let p1_path = map
        .entities
        .iter_mut()
        .find(|e| e.entity_type == EntityType::Path && e.owner == Some(Owner::new(0)))
        .unwrap()
        .path
        .as_ref()
        .unwrap();

    let y = 500.0;
    let x = 1000.0;
    let p2_path = p1_path
        .iter()
        .map(|p| NetVec2(Vec2::new(x - p.0.x, y - p.0.y)))
        .collect::<Vec<_>>();

    map.entities.iter_mut().for_each(|e| {
        if e.owner != Some(Owner::new(1)) {
            return;
        }
        if e.entity_type != EntityType::Path {
            continue;
        }
        e.path = Some(p2_path.clone());
    });

    dbg!(p1_path);
    dbg!(p2_path);

    Ok(())
}
