use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;
use bevy::asset::LoadState;
use crate::bottom_quad::BottomQuad;
use crate::shader::BillboardMaterial;

const PIXELS_PER_METER: f32 = 250.;

pub fn spawn_level(
    mut commands: Commands,
    level: Res<Handle<Level>>,
    mut level_assets: ResMut<Assets<Level>>,
    textures: Res<Handle<Textures>>,
    mut textures_assets: ResMut<Assets<Textures>>,
    mut state: ResMut<LevelLoadState>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut billboard_materials: ResMut<Assets<BillboardMaterial>>,
) {
    if *state == LevelLoadState::Loaded {
        return;
    }
    if asset_server.get_load_state(&*level) != LoadState::Loaded {
        return;
    }
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

    *state = LevelLoadState::Loaded;
    println!("Loading level done!");
}

#[derive(Serialize, Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "3d95a211-1b29-44a3-a9db-875cf44ff92c"]
pub struct Level {
    pub name: String,
    pub entities: Vec<LevelEntity>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelEntity {
    texture: String,
    position: [f32; 2],

    #[serde(default, rename = "type")]
    entity_type: EntityType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Sprite,
    Ground,
}

impl Default for EntityType {
    fn default() -> Self {
        EntityType::Sprite
    }
}

#[derive(Debug, Serialize, Deserialize, TypeUuid)]
#[uuid = "f1235a5a-89e5-463f-b531-d193e1a63870"]
pub struct Textures(pub Vec<TextureDefinition>);

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureDefinition {
    pub path: String,
    pub size: [u32; 2],
}

#[derive(Debug, Eq, PartialEq)]
pub enum LevelLoadState {
    Loading,
    Loaded,
}

