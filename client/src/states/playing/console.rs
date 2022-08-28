use crate::states::playing::floaty_text::FONT;
use bevy::prelude::*;
use std::collections::VecDeque;
use std::time::Duration;

// New text on the bottom.
// Slowly fades out?

// Also an event
#[derive(Component, Clone, Default, Debug)]
pub struct ConsoleItem {
    text: String,
    published: Duration,
}

impl ConsoleItem {
    pub fn new(text: String) -> Self {
        Self {
            text,
            ..Default::default()
        }
    }
}

pub fn handle_console_events(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut console_item_events: EventReader<ConsoleItem>,
) {
    for event in console_item_events.iter() {
        let mut event = event.clone();
        event.published = time.time_since_startup();
        commands
            .spawn_bundle(TextBundle {
                text: Text::from_section(
                    event.text.clone(),
                    TextStyle {
                        font: asset_server.load(FONT),
                        font_size: 25.0,
                        color: Color::rgba(0., 0., 0., 0.8),
                    },
                ),
                ..Default::default()
            })
            .insert(event.clone());
    }
}

pub fn update_console(
    mut commands: Commands,
    time: Res<Time>,
    mut lines: Query<(Entity, &ConsoleItem, &mut Text, &mut Style)>,
) {
    // Collect all the lines and sort by published time.
    let mut lines: Vec<_> = lines.iter_mut().collect();
    lines.sort_by_key(|(_, item, ..)| item.published);

    // Render from bottom upwards, fading out if it's longer than 5 seconds.
    let mut pos = 20.;
    for (entity, item, text, style) in lines.iter_mut().rev() {
        style.position_type = PositionType::Absolute;
        style.position.left = Val::Px(20.);
        style.position.bottom = Val::Px(pos);
        pos += 25.0;

        let age = time.time_since_startup() - item.published;
        let alpha = if age > Duration::from_secs(20) {
            let age = age - Duration::from_secs(20);
            let age = age.as_secs_f32();
            let age = age / 5.0;
            let age = age.min(1.0);
            1.0 - age
        } else {
            1.0
        };

        if alpha <= 0. {
            text.sections[0].value = "".to_string();
            commands.entity(*entity).despawn();
        } else {
            text.sections[0].value = item.text.clone();
            text.sections[0].style.color = Color::rgba(0., 0., 0., alpha);
        }
    }
}
