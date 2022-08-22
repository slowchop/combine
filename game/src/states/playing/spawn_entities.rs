use crate::app::{GameState, MyRaycastSet};
use crate::states::playing::bottom_quad::BottomQuad;
use crate::{
    shape, AlphaMode, AssetServer, Assets, BillboardMaterial, Color, Commands, EventReader, Handle,
    MaterialMeshBundle, Mesh, Quat, Res, ResMut, StandardMaterial, Textures, Vec2, YamlLevel,
};
use bevy::asset::LoadState;
use bevy_mod_raycast::RayCastMesh;
use iyes_loopless::prelude::NextState;
use shared::game::level::level_entity_transform;
use shared::game::managed_game::{EntityType, LevelEntity};
use std::f32::consts::TAU;

#[derive(Debug, Clone)]
pub struct SpawnEntity(pub LevelEntity);

pub fn spawn_entities(
    mut commands: Commands,
    textures: Res<Handle<Textures>>,
    textures_assets: ResMut<Assets<Textures>>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut billboard_materials: ResMut<Assets<BillboardMaterial>>,
    mut new_entities: EventReader<SpawnEntity>,
) {
    if asset_server.get_load_state(&*textures) != LoadState::Loaded {
        return;
    }

    let textures: &Textures = textures_assets.get(&textures).unwrap();
    for spawn in new_entities.iter() {
        dbg!(&spawn);
        let level_entity: &LevelEntity = &spawn.0;
        let texture_def = textures
            .0
            .iter()
            .find(|t| t.path == level_entity.texture)
            .ok_or_else(|| format!("Could not find {} in texture defs.", level_entity.texture))
            .unwrap();

        let mut transform = level_entity_transform(level_entity, texture_def);
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
