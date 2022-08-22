use crate::protocol::game_ready::GameReady;
use crate::protocol::position::Position;
use crate::protocol::request_tower_placement::RequestTowerPlacement;
use crate::{Auth, JoinFriendGame};
pub use join_random_game::JoinRandomGame;
use naia_shared::{
    Channel, ChannelDirection, ChannelMode, LinkConditionerConfig, OrderedReliableReceiver,
    Protocolize, ReliableSettings, SharedConfig, SocketConfig, TickBufferSettings,
};

pub mod auth;
pub mod game_ready;
pub mod join_friend_game;
pub mod join_random_game;
pub mod position;
pub mod request_tower_placement;

#[derive(Protocolize)]
pub enum Protocol {
    Position(Position),
    Auth(Auth),
    JoinRandomGame(JoinRandomGame),
    JoinFriendGame(JoinFriendGame),
    GameReady(GameReady),
    RequestTowerPlacement(RequestTowerPlacement),
}
