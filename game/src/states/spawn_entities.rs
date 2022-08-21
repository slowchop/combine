use crate::app::{GameState, MyRaycastSet};
use crate::states::playing::bottom_quad::BottomQuad;
use crate::states::playing::level::{EntityType, LevelEntity};
use crate::{
    shape, AlphaMode, AssetServer, Assets, BillboardMaterial, Color, Commands, EventReader, Handle,
    Level, MaterialMeshBundle, Mesh, Quat, Res, ResMut, StandardMaterial, Textures, Vec2,
};
use bevy::asset::LoadState;
use bevy_mod_raycast::RayCastMesh;
use iyes_loopless::prelude::NextState;
use std::f32::consts::TAU;

#[derive(Debug, Clone)]
pub struct SpawnEntity(pub LevelEntity);

pub fn spawn_entities(
    mut commands: Commands,
    level: Res<Handle<Level>>,
    textures: Res<Handle<Textures>>,
    textures_assets: ResMut<Assets<Textures>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut billboard_materials: ResMut<Assets<BillboardMaterial>>,
    mut new_entities: EventReader<SpawnEntity>,
) {
    println!("2 Waiting for level info to load...");
    if asset_server.get_load_state(&*level) != LoadState::Loaded {
        return;
    }

    println!("2 Waiting for texture info to load...");
    if asset_server.get_load_state(&*textures) != LoadState::Loaded {
        return;
    }

    let textures: &Textures = textures_assets.get(&textures).unwrap();
    for spawn in new_entities.iter() {
        let level_entity: &LevelEntity = &spawn.0;
        let texture_def = textures
            .0
            .iter()
            .find(|t| t.path == level_entity.texture)
            .ok_or_else(|| format!("Could not find {} in texture defs.", level_entity.texture))
            .unwrap();

        let mut transform = level_entity.transform(texture_def);
        match level_entity.entity_type {
            EntityType::Ground => {}
            _ => {
                transform.rotation = Quat::from_rotation_x(TAU * -0.125);
            }
        };

        let mesh = match level_entity.entity_type {
            EntityType::Ground => Mesh::from(shape::Plane { size: 10.0 }),
            _ => Mesh::from(BottomQuad {
                size: Vec2::new(1., 1.),
            }),
        };

        let alpha_mode = match level_entity.entity_type {
            EntityType::Ground => AlphaMode::Opaque,
            _ => AlphaMode::Blend,
        };

        let material = billboard_materials.add(BillboardMaterial {
            alpha_mode,
            color_texture: Some(asset_server.load(&level_entity.texture)),
            color: Color::ORANGE_RED,
        });

        let mut c = commands.spawn_bundle(MaterialMeshBundle {
            mesh: meshes.add(mesh),
            material,
            transform,
            ..Default::default()
        });

        if let EntityType::Ground = level_entity.entity_type {
            c.insert(RayCastMesh::<MyRaycastSet>::default());
        }
    }
}
