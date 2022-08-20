pub struct Settings {
    pub keyboard_scroll_speed: f32,
    pub mouse_scroll_speed: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            keyboard_scroll_speed: 15.0,
            mouse_scroll_speed: 20.0,
        }
    }
}
