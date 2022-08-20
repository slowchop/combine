use crate::settings::Settings;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct GameCamera {
    pub target: Vec2,
    pub zoom: f32,
}

impl Default for GameCamera {
    fn default() -> Self {
        Self {
            target: Vec2::ZERO,
            zoom: 50.0,
        }
    }
}

pub fn move_camera(
    settings: Res<Settings>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<MouseButton>>,
    mut motion_events: EventReader<MouseMotion>,
    mut scroll_events: EventReader<MouseWheel>,
    mut camera_query: Query<(&mut Transform, &mut GameCamera), With<Camera>>,
) {
    for (mut camera_transform, mut game_camera) in camera_query.iter_mut() {
        let mut pan = Vec2::ZERO;

        // Keyboard pan.
        if input.pressed(KeyCode::A) || input.pressed(KeyCode::Left) {
            pan.x -= settings.keyboard_scroll_speed;
        }
        if input.pressed(KeyCode::D) || input.pressed(KeyCode::Right) {
            pan.x += settings.keyboard_scroll_speed;
        }
        if input.pressed(KeyCode::W) || input.pressed(KeyCode::Up) {
            pan.y -= settings.keyboard_scroll_speed;
        }
        if input.pressed(KeyCode::S) || input.pressed(KeyCode::Down) {
            pan.y += settings.keyboard_scroll_speed;
        }

        // Mouse pan.
        if buttons.pressed(MouseButton::Right) {
            for mouse_motion in motion_events.iter() {
                pan -= (game_camera.zoom.sqrt() / 3.0) * mouse_motion.delta
            }
        }

        let mut zoom = 0f32;

        // Mouse zoom.
        for mouse_wheel in scroll_events.iter() {
            zoom -= mouse_wheel.y * settings.mouse_scroll_speed;
        }

        game_camera.zoom += zoom * game_camera.zoom * time.delta_seconds();
        if game_camera.zoom < 10.0 {
            game_camera.zoom += game_camera.zoom * 3.0 * time.delta_seconds();
        }
        if game_camera.zoom > 100.0 {
            game_camera.zoom -= game_camera.zoom * 3.0 * time.delta_seconds();
        }
        let zoom = game_camera.zoom;
        game_camera.target += pan * zoom.sqrt() * time.delta_seconds();

        let target = Vec3::new(game_camera.target.x, 0.0, game_camera.target.y);
        *camera_transform =
            Transform::from_translation(target + Vec3::new(0., game_camera.zoom, game_camera.zoom))
                .looking_at(target, Vec3::Y);
    }
}
