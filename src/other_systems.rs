use bevy::app::AppExit;
use bevy::prelude::*;

pub fn quit_on_escape(
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut exit: EventWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}
