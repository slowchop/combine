use crate::app::MyRaycastSet;
use crate::states::playing::floaty_text::{floaty_text_bundle, FONT};
use bevy::prelude::*;
use bevy_mod_raycast::Intersection;
use shared::game::defs::{CreepRef, Defs, TowerRef};
use shared::game::owner::Owner;
use shared::game::position::vec2_to_vec3;
use shared::game::ClientGameInfo;

#[derive(Component)]
pub struct TopHelperText(pub String);

pub fn top_helper_text(
    mut hover_stats: Query<(&TopHelperText, &mut Text, &mut Style)>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();

    for (top_helper_text, mut text, mut style) in hover_stats.get_single_mut() {
        let style: &mut Style = &mut style;
        text.sections[0].value = top_helper_text.0.to_string();
        style.position.top = Val::Px(150.0);
        style.size.width = Val::Px(400.0);
        style.position.left = Val::Px(window.width() as f32 / 2.0 - 200.0);
    }
}
