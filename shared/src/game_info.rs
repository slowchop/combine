use crate::player_name::PlayerName;

#[derive(Default)]
pub struct GameInfo {
    pub level: String,
    pub players: [PlayerName; 2],
    pub you_are: usize,
}
