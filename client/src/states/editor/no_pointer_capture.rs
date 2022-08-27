use bevy::math::swizzles::Vec3Swizzles;
use bevy::prelude::*;

pub struct IsPointerCaptured(pub bool);

#[derive(Component)]
pub struct NoPointerCapture;

pub fn is_pointer_captured_system(
    windows: Res<Windows>,
    mut is_pointer_captured: ResMut<IsPointerCaptured>,
    node_query: Query<(&Node, &GlobalTransform, &Visibility), Without<NoPointerCapture>>,
) {
    // TODO: Doesn't work...
    // https://old.reddit.com/r/bevy/comments/vbll6b/capturing_mouse_clicks_in_the_ui_before_they_get/

    // is_pointer_captured.0 = windows
    //     .get_primary()
    //     .and_then(|window| window.cursor_position())
    //     .map(|pointer_position| {
    //         node_query
    //             .iter()
    //             .filter(|(_, _, &Visibility { is_visible })| is_visible)
    //             .any(|(&Node { size }, &global_transform, ..)| {
    //                 let translation = global_transform.translation();
    //                 let node_position = translation.xy();
    //                 let half_size = 0.5 * size;
    //                 let min = node_position - half_size;
    //                 let max = node_position + half_size;
    //                 (min.x..max.x).contains(&pointer_position.x)
    //                     && (min.y..max.y).contains(&pointer_position.y)
    //             })
    //     })
    //     .unwrap_or(false);
}
