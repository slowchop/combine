use crate::states::map_editor::input_events::Draggable;
use crate::states::map_editor::load_map::PathInfo;
use crate::states::map_editor::menu::{AddPathEvent, EditorInfo};
use crate::states::playing::bottom_quad::BottomQuad;
use crate::BillboardMaterial;
use bevy::prelude::*;
use shared::game::defs::{Defs, EntityType};
use shared::game::position::vec2_to_vec3;

pub fn add_path(
    mut commands: Commands,
    mut path_events: EventReader<AddPathEvent>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut billboard_materials: ResMut<Assets<BillboardMaterial>>,
    mut defs: ResMut<Defs>,
    editor_info: Res<EditorInfo>,
) {
    let level_def = match defs.levels.get_mut(&editor_info.map_name) {
        Some(m) => m,
        None => {
            return;
        }
    };

    for event in path_events.iter() {
        let owner = event.0;
        let entity_def = level_def.get_path(owner);
        let texture = "editor/path-waypoint.png";
        let mut path = entity_def.path.as_mut().unwrap();
        let idx = path.len();
        let waypoint: Vec2 = path.last().unwrap().into();
        let waypoint: Vec2 = waypoint + Vec2::new(10.0, 10.0);
        path.push(waypoint.clone().into());

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
                transform: Transform::from_translation(vec2_to_vec3(&waypoint.into()).into()),
                ..Default::default()
            })
            .insert(Draggable)
            .insert(PathInfo {
                owner: owner.clone(),
                index: idx,
            });
    }
}
