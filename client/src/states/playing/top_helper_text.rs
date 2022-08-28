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

pub fn top_helper_text(mut hover_stats: Query<(&TopHelperText, &mut Text, &mut Style)>) {
    for (top_helper_text, mut text, mut style) in hover_stats.get_single_mut() {
        text.sections[0].value = top_helper_text.0.to_string();
    }
}
