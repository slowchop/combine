use crate::app::MyRaycastSet;
use crate::states::map_editor::input_events::Draggable;
use crate::states::map_editor::menu::{ClearEditorLevelEvent, EditorInfo, LoadEvent};
use crate::states::playing::bottom_quad::BottomQuad;
use crate::states::playing::console::ConsoleItem;
use crate::BillboardMaterial;
use bevy::prelude::*;
use bevy_mod_raycast::RayCastMesh;
use bevy_prototype_lyon::prelude::tess::path::Position;
use shared::game::defs::{Defs, EntityDef, EntityType, LevelDef};
use shared::game::owner::Owner;
use shared::game::position::vec2_to_vec3;
use shared::game::shared_game::ServerEntityId;
use std::f32::consts::TAU;

pub struct CreateEditorEntity(pub EntityDef);

#[derive(Component, Debug, Clone)]
pub struct PathInfo {
    pub owner: Owner,
    pub index: usize,
}

pub fn load_map(
    mut console: EventWriter<ConsoleItem>,
    defs: Res<Defs>,
    mut load_map_events: EventReader<LoadEvent>,
    mut create_editor_entities: EventWriter<CreateEditorEntity>,
) {
    for map in load_map_events.iter() {
        dbg!(&map.0);
        let level_def = match defs.levels.get(&map.0) {
            None => {
                console.send(ConsoleItem::new(format!(
                    "Could not find map: \"{}\"",
                    map.0
                )));
                return;
            }
            Some(m) => m,
        };

        for (idx, entity_def) in level_def.entities.iter().enumerate() {
            let mut entity_def = entity_def.clone();
            // Hackily adding an ID so that we can delete entities in the editor without
            // reordering the level.entities vec.
            entity_def.server_entity_id = Some(ServerEntityId(idx as u32));
            create_editor_entities.send(CreateEditorEntity(entity_def.clone()));
        }
    }
}

pub fn create_editor_entities(
    mut create_editor_entities: EventReader<CreateEditorEntity>,
    mut commands: Commands,
    defs: Res<Defs>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut billboard_materials: ResMut<Assets<BillboardMaterial>>,
) {
    {
        for create_editor_entity in create_editor_entities.iter() {
            let entity_def = &create_editor_entity.0;

            let mut texture = &entity_def.texture;

            if entity_def.entity_type == EntityType::Guide {
                commands.spawn().insert(entity_def.clone());
                continue;
            }

            let home = Some("editor/home.png".to_string());
            if entity_def.entity_type == EntityType::Base {
                texture = &home;
            }

            let spawn = Some("editor/spawn-point.png".to_string());
            if entity_def.entity_type == EntityType::Spawn {
                texture = &spawn;
            }

            if entity_def.entity_type == EntityType::Path {
                let owner = entity_def.owner.as_ref().unwrap();
                let path = entity_def.path.as_ref().unwrap();
                for (idx, waypoint) in path.iter().enumerate() {
                    let texture = if idx == 0 {
                        "editor/path-start.png"
                    } else {
                        "editor/path-waypoint.png"
                    };
                    let material = billboard_materials.add(BillboardMaterial {
                        alpha_mode: AlphaMode::Blend,
                        color_texture: Some(asset_server.load(texture)),
                        owner: owner.0 as i32,
                        color: Color::WHITE,
                    });
                    let mesh = Mesh::from(BottomQuad {
                        size: Vec2::new(1., 1.),
                    });

                    commands
                        .spawn_bundle(MaterialMeshBundle {
                            mesh: meshes.add(mesh),
                            material,
                            transform: Transform::from_translation(
                                vec2_to_vec3(&waypoint.into()).into(),
                            ),
                            ..Default::default()
                        })
                        .insert(Draggable)
                        .insert(PathInfo {
                            owner: owner.clone(),
                            index: idx,
                        });
                }

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
                EntityType::Sprite => {
                    entity.insert(Name::new("Sprite")).insert(Draggable);
                }
                EntityType::Spawn => {
                    entity.insert(Name::new("Spawn")).insert(Draggable);
                }
                EntityType::Base => {
                    entity.insert(Name::new("Base")).insert(Draggable);
                }
                EntityType::Path => {
                    entity.insert(Name::new("Path"));
                }
                EntityType::Tower => {}
                EntityType::Creep => {}
                EntityType::Guide => {}
            }

            entity.insert(entity_def.clone());
        }
    }
}
