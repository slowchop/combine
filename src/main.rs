mod level;
mod bottom_quad;
mod shader;
mod textures;

use bevy::app::AppExit;
use bevy::asset::AssetServerSettings;
use bevy::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;
use crate::level::{Level, LevelLoadState, spawn_level, TextureDefinition, Textures};
use crate::textures::update_texture_sizes;
use clap::Parser;
use crate::shader::BillboardMaterial;
use bevy_inspector_egui::WorldInspectorPlugin;

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
        None => play(),
        Some(Command::UpdateTextureSizes) => update_texture_sizes()?,
    }
    Ok(())
}

fn play() {
    App::new()
        .insert_resource(WindowDescriptor {
            resizable: false,
            width: 1024f32,
            height: 768f32,
            title: "Combined Towers".to_string(),
            ..Default::default()
        })
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.1, 0.3, 0.4)))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(AmbientLight {
            color: Color::BISQUE,
            brightness: 0.2,
        })
        .insert_resource(LevelLoadState::Loading)
        .add_plugins(DefaultPlugins)
        .add_plugin(YamlAssetPlugin::<Textures>::new(&["textures"]))
        .add_plugin(YamlAssetPlugin::<Level>::new(&["level"]))
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(MaterialPlugin::<BillboardMaterial>::default())
        .add_startup_system(init)
        .add_system(quit_on_escape)
        .add_system(spawn_level)
        .run();
}

fn quit_on_escape(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(0., 10., 10.).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.insert_resource(asset_server.load::<Textures, _>("game.textures"));
    commands.insert_resource(asset_server.load::<Level, _>("levels/test.level"));
}