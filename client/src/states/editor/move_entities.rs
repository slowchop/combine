use crate::app::MyRaycastSet;
use crate::states::editor::load_map::PathInfo;
use crate::states::editor::menu::EditorInfo;
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use shared::game::defs::{Defs, EntityDef, EntityType};
use shared::game::position::vec2_to_vec3;

#[derive(Component)]
pub struct Draggable;

#[derive(Default)]
pub enum EditorDragState {
    #[default]
    NotDragging,
    Dragging {
        entity: Entity,
    },
}

pub fn input_events(
    mut commands: Commands,
    mut defs: ResMut<Defs>,
    editor_info: Res<EditorInfo>,
    mut query: Query<
        (
            Entity,
            &mut Transform,
            Option<&mut EntityDef>,
            Option<&mut PathInfo>,
        ),
        With<Draggable>,
    >,
    intersection_query: Query<&Intersection<MyRaycastSet>>,
    buttons: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    mut drag_state: ResMut<EditorDragState>,
) {
    let level_def = match defs.levels.get_mut(&editor_info.map_name) {
        Some(m) => m,
        None => {
            return;
        }
    };

    let mut position = None;
    for intersection in intersection_query.iter() {
        let intersection = if let Some(i) = intersection.position() {
            i
        } else {
            continue;
        };
        position = Some(Vec2::new(intersection.x, intersection.z));
    }
    let position = if let Some(p) = position {
        p
    } else {
        return;
    };

    let mouse_position = vec2_to_vec3(&position);

    match *drag_state {
        EditorDragState::NotDragging => {
            // Find closest entity from mouse_position
            let mut closest_entity = None;
            let mut closest_distance = None;
            for (entity, transform, maybe_entity_def, _) in query.iter() {
                if let Some(entity_def) = maybe_entity_def {
                    if entity_def.entity_type == EntityType::Spawn
                        || entity_def.entity_type == EntityType::Base
                        || entity_def.entity_type == EntityType::Guide
                        || entity_def.entity_type == EntityType::Ground
                    {
                        continue;
                    }
                }

                let distance = (transform.translation - mouse_position).length();
                if let Some(d) = closest_distance {
                    if distance < d {
                        closest_distance = Some(distance);
                        closest_entity = Some(entity);
                    }
                } else {
                    closest_distance = Some(distance);
                    closest_entity = Some(entity);
                }
            }
            if closest_entity.is_none() {
                return;
            }

            if keys.just_released(KeyCode::Delete) {
                let (entity, transform, maybe_entity_def, maybe_path_info) =
                    query.get(closest_entity.unwrap()).unwrap();

                if let Some(entity_def) = maybe_entity_def {
                    if entity_def.entity_type == EntityType::Path
                        || entity_def.entity_type == EntityType::Spawn
                        || entity_def.entity_type == EntityType::Base
                        || entity_def.entity_type == EntityType::Guide
                        || entity_def.entity_type == EntityType::Ground
                    {
                        return;
                    }
                } else if let Some(path_info) = maybe_path_info {
                    let entity_def = level_def
                        .entities
                        .iter_mut()
                        .find(|e| {
                            if e.entity_type != EntityType::Path {
                                return false;
                            }

                            if e.owner != Some(path_info.owner) {
                                return false;
                            }

                            return true;
                        })
                        .unwrap();

                    let path = entity_def.path.as_mut().unwrap();
                }

                commands.entity(entity).despawn();
            } else if buttons.just_pressed(MouseButton::Left) {
                if let Some(entity) = closest_entity {
                    *drag_state = EditorDragState::Dragging { entity };
                }
            }
        }
        EditorDragState::Dragging { entity } => {
            let (_, mut transform, ref mut maybe_entity_def, ref mut maybe_path_info) =
                query.get_mut(entity).unwrap();

            if buttons.just_released(MouseButton::Left) {
                *drag_state = EditorDragState::NotDragging;
            } else {
                transform.translation = mouse_position;
                if let Some(entity_def) = maybe_entity_def {
                    entity_def.position = Some(position.into());
                }
                if let Some(path_info) = maybe_path_info {
                    let entity_def = level_def
                        .entities
                        .iter_mut()
                        .find(|e| {
                            if e.entity_type != EntityType::Path {
                                return false;
                            }

                            if e.owner != Some(path_info.owner) {
                                return false;
                            }

                            return true;
                        })
                        .unwrap();

                    let path_waypoints = entity_def.path.as_mut().unwrap();
                    path_waypoints[path_info.index] = position.into();
                }
            }
        }
    }
}