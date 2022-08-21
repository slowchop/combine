use crate::app::GameState;
use crate::states::playing::bottom_quad::BottomQuad;
use crate::states::playing::level::{EntityType, PIXELS_PER_METER};
use crate::{BillboardMaterial, Level, Textures};
use bevy::asset::LoadState;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use shared::game_info::GameInfo;
use std::f32::consts::TAU;

pub fn init(
    mut commands: Commands,
    time: Res<Time>,
    game_info: Res<GameInfo>,
    asset_server: Res<AssetServer>,
) {
    let level_path = format!("levels/{}.level", game_info.level);
    println!("Loading level... {}", level_path);

    commands.insert_resource(asset_server.load::<Textures, _>("game.textures"));
    commands.insert_resource(asset_server.load::<Level, _>(&level_path));
}

pub fn spawn_level(
    mut commands: Commands,
    level: Res<Handle<Level>>,
    mut level_assets: ResMut<Assets<Level>>,
    textures: Res<Handle<Textures>>,
    mut textures_assets: ResMut<Assets<Textures>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut billboard_materials: ResMut<Assets<BillboardMaterial>>,
) {
    println!("Waiting for level info to load...");
    if asset_server.get_load_state(&*level) != LoadState::Loaded {
        return;
    }

    println!("Waiting for texture info to load...");
    if asset_server.get_load_state(&*textures) != LoadState::Loaded {
        return;
    }

    println!("Loading level...");
    let textures: &Textures = textures_assets.get(&textures).unwrap();
    let level: &Level = level_assets.get(&level).unwrap();
    for e in &level.entities {
        let texture_def = textures
            .0
            .iter()
            .find(|t| t.path == e.texture)
            .ok_or_else(|| format!("Could not find {} in texture defs.", e.texture))
            .unwrap();

        let x = e.position[0];
        let y = e.position[1];
        let mut transform = Transform::from_xyz(x, 0., y).with_scale(Vec3::new(
            texture_def.size[0] as f32 / PIXELS_PER_METER,
            texture_def.size[1] as f32 / PIXELS_PER_METER,
            1.0,
        ));
        match e.entity_type {
            EntityType::Ground => {}
            _ => {
                transform.rotation = Quat::from_rotation_x(TAU * -0.125);
            }
        };

        let mesh = match e.entity_type {
            EntityType::Ground => Mesh::from(shape::Plane { size: 10.0 }),
            _ => Mesh::from(BottomQuad {
                size: Vec2::new(1., 1.),
            }),
        };

        let alpha_mode = match e.entity_type {
            EntityType::Ground => AlphaMode::Opaque,
            _ => AlphaMode::Blend,
        };

        let material = billboard_materials.add(BillboardMaterial {
            alpha_mode,
            color_texture: Some(asset_server.load(&e.texture)),
            color: Color::ORANGE_RED,
        });

        commands.spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material,
            transform,
            ..Default::default()
        });
    }

    commands.insert_resource(NextState(GameState::Playing));
    println!("Loading level done!");
}
