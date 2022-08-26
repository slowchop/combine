use crate::app::{GameState, MyRaycastSet};
use crate::states::playing::bottom_quad::BottomQuad;
use crate::states::playing::left_click::Guide;
use crate::{
    shape, AlphaMode, AssetServer, Assets, BillboardMaterial, Color, Commands, EventReader, Handle,
    MaterialMeshBundle, Mesh, Quat, Res, ResMut, StandardMaterial, Vec2,
};
use bevy::prelude::*;
use bevy_mod_raycast::RayCastMesh;
use bevy_prototype_lyon::prelude::{
    DrawMode, FillMode, GeometryBuilder, LineCap, LineJoin, StrokeMode, StrokeOptions,
};
use bevy_prototype_lyon::shapes;
use bevy_prototype_lyon::shapes::Polygon;
use shared::game::defs::{CreepRef, Defs, EntityDef, EntityType, TowerRef, PIXELS_PER_METER};
use shared::game::path::Path;
use shared::game::position::vec2_to_vec3;
use shared::game::shared_game::{ServerEntityId, SharedGame};
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
    mut game: Query<&mut SharedGame>,
) {
    let mut game = if let Ok(g) = game.get_single_mut() {
        g
    } else {
        warn!("Could not get game in spawn_entities!");
        return;
    };

    for spawn in new_entities.iter() {
        let entity_def = &spawn.entity_def;
        let mut texture = entity_def.texture.clone();

        if entity_def.entity_type == EntityType::Spawn {
            warn!("TODO: Ignoring base for now.");
            continue;
        }
        if entity_def.entity_type == EntityType::Base {
            warn!("TODO: Ignoring base for now.");
            continue;
        }

        if let EntityType::Path = entity_def.entity_type {
            // Client doesn't care about path.
            // Server just spams client with spawn + position updates.

            let path = if let Some(p) = &entity_def.path {
                p
            } else {
                warn!("Path entity has no path!");
                continue;
            };
            let owner = if let Some(o) = entity_def.owner {
                o
            } else {
                warn!("Path entity has no owner!");
                continue;
            };

            let path: Vec<Vec3> = path.iter().map(|p| vec2_to_vec3(&p.into())).collect();
            game.paths.insert(owner, Path(path));

            // Debugging down here.
            //
            // let shape = shapes::RegularPolygon {
            //     sides: 6,
            //     feature: shapes::RegularPolygonFeature::Radius(200.0),
            //     ..shapes::RegularPolygon::default()
            // };
            //
            // commands.spawn_bundle(GeometryBuilder::build_as(
            //     &shape,
            //     DrawMode::Outlined {
            //         fill_mode: FillMode::color(Color::CYAN),
            //         outline_mode: StrokeMode::new(Color::BLACK, 10.0),
            //     },
            //     Transform::default().with_rotation(Quat::from_rotation_z(TAU * 0.75)),
            // ));
            //
            // // TODO: These lines aren't being drawn
            //
            // let path = entity_def.path.as_ref().unwrap();
            // let path = path
            //     .iter()
            //     .map(|p| p.into())
            //     .map(|p: Vec2| p * PIXELS_PER_METER)
            //     .collect::<Vec<Vec2>>();
            // let shape = Polygon {
            //     points: path,
            //     closed: false,
            // };
            //
            // commands.spawn_bundle(GeometryBuilder::build_as(
            //     &shape,
            //     DrawMode::Stroke(StrokeMode::new(Color::BLACK, 10.0)),
            //     Default::default(),
            // ));

            continue;
        }

        let mesh = match entity_def.entity_type {
            EntityType::Ground => Mesh::from(shape::Plane { size: 10000.0 }),
            EntityType::Guide => Mesh::from(shape::Plane { size: 10.0 }),
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
                warn!("Creep not found while spawning: {:?}", entity_def);
                continue;
            };
            let creep = if let Some(t) = defs.creeps.get(creep_name) {
                t
            } else {
                warn!("Creep not found while spawning 2: {:?}", entity_def);
                continue;
            };

            texture = Some(creep.texture.clone());
        };

        let material = texture.as_ref().map(|texture_name| {
            let shader_owner = if let Some(o) = entity_def.owner {
                o.0 as i32
            } else {
                -1
            };

            billboard_materials.add(BillboardMaterial {
                alpha_mode,
                color_texture: Some(asset_server.load(texture_name.as_str())),
                color: Color::WHITE,
                owner: shader_owner,
            })
        });

        let transform: Option<Transform> = defs
            .level_entity_transform(&texture, &entity_def.position.as_ref().map(|p| p.into()))
            .map(|mut transform| match entity_def.entity_type {
                EntityType::Ground => transform,
                EntityType::Guide => {
                    let pos = transform.translation + Vec3::new(0.5, 0.5, 0.5);
                    transform.translation = pos;
                    transform
                }
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

        match entity_def.entity_type {
            EntityType::Ground => {
                entity
                    .insert(Name::new("Ground"))
                    .insert(RayCastMesh::<MyRaycastSet>::default());
            }
            EntityType::Guide => {
                entity.insert(Name::new("Guide")).insert(Guide);
            }
            EntityType::Tower => {
                let owner = if let Some(o) = entity_def.owner {
                    o
                } else {
                    warn!("Tower entity has no owner!");
                    continue;
                };

                // Already checked
                let tower_ref = entity_def.tower.as_ref().unwrap();
                // let tower = defs.towers.get(&tower_name).unwrap(); // Already checked
                entity
                    .insert(Name::new(format!("Tower {:?}", tower_ref)))
                    .insert(tower_ref.to_owned())
                    .insert(owner);
            }
            EntityType::Creep => {
                let owner = if let Some(o) = entity_def.owner {
                    o
                } else {
                    warn!("Tower entity has no owner!");
                    continue;
                };

                // Already checked
                let creep_ref = entity_def.creep.as_ref().unwrap();
                entity
                    .insert(Name::new(format!("Creep {:?}", creep_ref)))
                    .insert(creep_ref.to_owned())
                    .insert(owner);
            }
            EntityType::Sprite => {
                entity.insert(Name::new("Sprite"));
            }
            EntityType::Spawn => {
                entity.insert(Name::new("Spawn"));
            }
            EntityType::Base => {
                entity.insert(Name::new("Base"));
            }
            EntityType::Path => {
                entity.insert(Name::new("Path"));
            }
        }

        if let Some(server_entity_id) = &spawn.server_entity_id {
            println!("Inserting server entity id: {:?}", server_entity_id);
            entity.insert(*server_entity_id);
            game.client_add_entity(*server_entity_id, entity.id());
        } else {
            if spawn.entity_def.entity_type == EntityType::Creep {
                warn!("Spawning: No server entity id for {:?}", spawn);
            }
        }
    }
}
