use crate::app::GameState;

pub mod connecting;
pub mod loading_level;
pub mod main_menu;
pub mod playing;
pub mod splash;
pub mod waiting_for_random;

/// Used when connecting, to know where to go after the Connecting state.
#[derive(Debug)]
pub struct ContinueState(pub Option<GameState>);
