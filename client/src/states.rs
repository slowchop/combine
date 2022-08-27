pub mod attr_editor;
pub mod connecting;
pub mod disconnected;
pub mod loading_level;
pub mod main_menu;
pub mod map_editor;
pub mod playing;
pub mod splash;
pub mod waiting_for_random;

use crate::app::GameState;

/// Used when connecting, to know where to go after the Connecting state.
#[derive(Debug)]
pub struct ContinueState(pub Option<GameState>);
