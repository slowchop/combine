pub struct Settings {
    pub keyboard_scroll_speed: f32,
    pub mouse_scroll_speed: f32,
    pub start_multiplayer_immediately: bool,
    pub start_map_editor_immediately: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            keyboard_scroll_speed: 15.0,
            mouse_scroll_speed: 20.0,
            start_multiplayer_immediately: false,
            start_map_editor_immediately: false,
        }
    }
}
