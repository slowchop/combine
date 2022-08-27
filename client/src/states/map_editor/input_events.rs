use crate::app::MyRaycastSet;
use crate::states::map_editor::load_map::PathInfo;
use crate::states::map_editor::menu::EditorInfo;
use crate::states::map_editor::no_pointer_capture::IsPointerCaptured;
use crate::states::playing::console::ConsoleItem;
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use shared::game::defs::{Defs, EntityDef, EntityType};
use shared::game::position::vec2_to_vec3;
use std::time::Duration;

#[derive(Component)]
pub struct Draggable;

#[derive(Default)]
pub enum EditorDragState {
    #[default]
    NotDragging,
    Dragging {
        start_time: Duration,
        original_position: Vec2,
        entity: Entity,
    },
}

pub fn input_events(
    time: Res<Time>,
    mut console: EventWriter<ConsoleItem>,
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
    let mut position = if let Some(p) = position {
        p
    } else {
        return;
    };

    let mut mouse_position = vec2_to_vec3(&position);

    match *drag_state {
        EditorDragState::NotDragging => {
            // Find closest entity from mouse_position
            let mut closest_entity = None;
            let mut closest_distance = None;
            for (entity, transform, maybe_entity_def, _) in query.iter() {
                if let Some(entity_def) = maybe_entity_def {
                    if entity_def.entity_type == EntityType::Guide
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
                if keys.just_released(KeyCode::Delete) {
                    console.send(ConsoleItem::new("Nothing can be deleted!".to_string()));
                }
                return;
            }
            if closest_distance.unwrap() > 2.0 {
                if keys.just_released(KeyCode::Delete) {
                    console.send(ConsoleItem::new("Nothing nearby to delete.".to_string()));
                }
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

                    let server_id = maybe_entity_def.unwrap().server_entity_id.unwrap();
                    level_def
                        .entities
                        .retain(|e| e.server_entity_id != Some(server_id));

                    console.send(ConsoleItem::new(format!("Deleted {:?}", entity_def)));
                } else if let Some(path_info) = maybe_path_info {
                    console.send(ConsoleItem::new(
                        "TODO: Can't remove paths yet.".to_string(),
                    ));
                    return;
                }

                commands.entity(entity).despawn();
            } else if buttons.just_pressed(MouseButton::Left) {
                console.send(ConsoleItem::new(format!(
                    "Dragging {}.",
                    closest_entity.unwrap().id()
                )));

                if let Some(entity) = closest_entity {
                    *drag_state = EditorDragState::Dragging {
                        entity,
                        start_time: time.time_since_startup(),
                        original_position: position,
                    };
                }
            }
        }
        EditorDragState::Dragging {
            entity,
            start_time,
            original_position,
        } => {
            let (_, mut transform, ref mut maybe_entity_def, ref mut maybe_path_info) =
                query.get_mut(entity).unwrap();

            if buttons.just_released(MouseButton::Left) {
                *drag_state = EditorDragState::NotDragging;
                // if time.time_since_startup() - start_time < Duration::from_secs_f32(0.4) {
                // println!("Drag time was too short, ignoring.");
                // mouse_position = vec2_to_vec3(&original_position);
                // position = original_position;
                // } else {
                return;
                // }
            }

            transform.translation = mouse_position;
            if let Some(entity_def) = maybe_entity_def {
                // Update the entity's position in the game.
                entity_def.position = Some(position.into());

                // Update the entity's position inside level_def.
                // Get the entity_def from the level_def.
                let entity_def = level_def
                    .entities
                    .iter_mut()
                    .find(|e| e.server_entity_id == entity_def.server_entity_id)
                    .unwrap();
                entity_def.position = Some(position.into());

                println!("Updated entity def position");
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
