use bevy::prelude::*;

#[derive(Component, Default)]
pub struct FloatyText {
    pub text: String,
    pub world_position: Vec3,
}

pub fn update_floaty_text_and_world_to_screen_pos(
    camera_query: Query<(&GlobalTransform, &Camera)>,
    mut query: Query<(&FloatyText, &mut Style, &mut Text)>,
) {
    let (camera_transform, camera) = camera_query.single();

    for (floaty, mut style, mut text) in query.iter_mut() {
        let viewport_pos = match camera.world_to_viewport(camera_transform, floaty.world_position) {
            Some(pos) => pos,
            None => continue,
        };

        style.position.left = Val::Px(viewport_pos.x);
        style.position.bottom = Val::Px(viewport_pos.y);
        text.sections[0].value = floaty.text.clone();
    }
}
