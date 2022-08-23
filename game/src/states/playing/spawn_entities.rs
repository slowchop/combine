use crate::app::{GameState, MyRaycastSet};
use crate::states::playing::bottom_quad::BottomQuad;
use crate::{
    shape, AlphaMode, AssetServer, Assets, BillboardMaterial, Color, Commands, EventReader, Handle,
    MaterialMeshBundle, Mesh, Quat, Res, ResMut, StandardMaterial, Vec2,
};
use bevy::prelude::*;
use bevy_mod_raycast::RayCastMesh;
use shared::game::components::Damage;
use shared::game::defs::{CreepRef, Defs, EntityDef, EntityType, TowerRef};
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
        let entity_def = &spawn.entity_def;
        let mut texture = entity_def.texture.clone();

        match entity_def.entity_type {
            EntityType::Path | EntityType::Spawn => {
                warn!(?entity_def.entity_type, "TODO");
                continue;
            }
            _ => {}
        }

        let mesh = match entity_def.entity_type {
            EntityType::Ground => Mesh::from(shape::Plane { size: 10.0 }),
            _ => Mesh::from(BottomQuad {
                size: Vec2::new(1., 1.),
            }),
        };
        let alpha_mode = match entity_def.entity_type {
            EntityType::Ground => AlphaMode::Opaque,
            _ => AlphaMode::Blend,
        };

        if entity_def.entity_type == EntityType::Tower {
            if texture.is_some() {
                warn!(
                    "Texture was already specified for spawning tower: {:?}",
                    entity_def
                );
            }
            let tower_name = if let Some(t) = &entity_def.tower {
                t
            } else {
                warn!("Tower not found: {:?}", entity_def);
                continue;
            };
            let tower = if let Some(t) = defs.towers.get(tower_name) {
                t
            } else {
                warn!("Tower not found: {:?}", entity_def);
                continue;
            };

            texture = Some(tower.texture.clone());
        };

        if entity_def.entity_type == EntityType::Creep {
            if texture.is_some() {
                warn!(
                    "Texture was already specified for spawning ecreep: {:?}",
                    entity_def
                );
            }
            let creep_name = if let Some(t) = &entity_def.creep {
                t
            } else {
                warn!("Creep not found: {:?}", entity_def);
                continue;
            };
            let creep = if let Some(t) = defs.creeps.get(creep_name) {
                t
            } else {
                warn!("Creep not found: {:?}", entity_def);
                continue;
            };

            texture = Some(creep.texture.clone());
        };

        let material = texture.as_ref().map(|texture_name| {
            billboard_materials.add(BillboardMaterial {
                alpha_mode,
                color_texture: Some(asset_server.load(texture_name.as_str())),
                color: Color::WHITE,
            })
        });

        let transform: Option<Transform> = defs
            .level_entity_transform(&texture, &entity_def.position.as_ref().map(|p| p.into()))
            .map(|mut transform| match entity_def.entity_type {
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
                warn!("no transform and/or material for entity {:?}", entity_def);
                continue;
            }
        };

        if let EntityType::Ground = entity_def.entity_type {}

        match entity_def.entity_type {
            EntityType::Ground => {
                entity.insert(RayCastMesh::<MyRaycastSet>::default());
            }
            EntityType::Tower => {
                // Already checked
                let tower_name = entity_def.tower.as_ref().unwrap().to_string();
                // let tower = defs.towers.get(&tower_name).unwrap(); // Already checked
                entity.insert(TowerRef(tower_name));
            }
            EntityType::Creep => {
                // Already checked
                let creep_name = entity_def.creep.as_ref().unwrap().to_string();
                entity.insert(CreepRef(creep_name));
            }
            _ => {}
        }
    }
}
