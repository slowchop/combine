use naia_bevy_server::RoomKey;
use shared::game::player_name::PlayerName;

pub struct ServerPlayer {
    pub name: PlayerName,
    pub room: RoomKey,
}
