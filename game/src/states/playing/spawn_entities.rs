use crate::app::{GameState, MyRaycastSet};
use crate::states::playing::bottom_quad::BottomQuad;
use crate::{
    shape, AlphaMode, AssetServer, Assets, BillboardMaterial, Color, Commands, EventReader, Handle,
    MaterialMeshBundle, Mesh, Quat, Res, ResMut, StandardMaterial, Vec2,
};
use bevy::prelude::*;
use bevy_mod_raycast::RayCastMesh;
use shared::game::defs::{Defs, EntityDef, EntityType};
use shared::game::shared_game::ServerEntityId;
use std::f32::consts::TAU;

#[derive(Debug, Clone)]
pub struct SpawnEntityEvent {
    pub server_entity_id: Option<ServerEntityId>,
    pub entity_def: EntityDef,
}

pub fn spawn_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut billboard_materials: ResMut<Assets<BillboardMaterial>>,
    mut new_entities: EventReader<SpawnEntityEvent>,
    defs: Res<Defs>,
) {
    for spawn in new_entities.iter() {
        let level_entity: &EntityDef = &spawn.entity_def;

        match level_entity.entity_type {
            EntityType::Path => {
                warn!("TODO: Path");
                return;
            }
            _ => {}
        }

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

        let material = level_entity.texture.as_ref().map(|texture_name| {
            billboard_materials.add(BillboardMaterial {
                alpha_mode,
                color_texture: Some(asset_server.load(texture_name.as_str())),
                color: Color::WHITE,
            })
        });

        let transform: Option<Transform> =
            defs.level_entity_transform(level_entity)
                .map(|mut transform| match level_entity.entity_type {
                    EntityType::Ground => transform,
                    _ => {
                        transform.rotation = Quat::from_rotation_x(TAU * -0.125);
                        transform
                    }
                });

        let mut entity = match (transform, material) {
            (Some(transform), Some(material)) => commands.spawn_bundle(MaterialMeshBundle {
                mesh: meshes.add(mesh),
                material,
                transform,
                ..Default::default()
            }),
            _ => {
                warn!("no transform and/or material for entity {:?}", level_entity);
                continue;
            }
        };

        if let EntityType::Ground = level_entity.entity_type {
            entity.insert(RayCastMesh::<MyRaycastSet>::default());
        }

        // match level_entity.entity_type {
        //     EntityType::Ground => {}
        //     _ => {
        //         let texture_def = defs.textures.get(&level_entity.texture).unwrap();
        //         let mut transform = level_entity_transform(level_entity, texture_def).unwrap();
        //         transform.rotation = Quat::from_rotation_x(TAU * -0.125);
        //     }
        // };
        //
        // let mesh = match level_entity.entity_type {
        //     EntityType::Ground => Mesh::from(shape::Plane { size: 10.0 }),
        //     _ => Mesh::from(BottomQuad {
        //         size: Vec2::new(1., 1.),
        //     }),
        // };
        //
        // let alpha_mode = match level_entity.entity_type {
        //     EntityType::Ground => AlphaMode::Opaque,
        //     _ => AlphaMode::Blend,
        // };
        //
        // let material = billboard_materials.add(BillboardMaterial {
        //     alpha_mode,
        //     color_texture: Some(asset_server.load(&level_entity.texture)),
        //     color: Color::ORANGE_RED,
        // });
        //
        // let mut c = commands.spawn_bundle(MaterialMeshBundle {
        //     mesh: meshes.add(mesh),
        //     material,
        //     transform,
        //     ..Default::default()
        // });
        //
        // if let EntityType::Ground = level_entity.entity_type {
        //     c.insert(RayCastMesh::<MyRaycastSet>::default());
        // }
    }
}
