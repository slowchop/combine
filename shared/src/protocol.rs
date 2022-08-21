use crate::protocol::game_ready::GameReady;
use crate::protocol::place_tower::PlaceTower;
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
pub mod place_tower;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    JoinRandomGame(JoinRandomGame),
    JoinFriendGame(JoinFriendGame),
    GameReady(GameReady),
    PlaceTower(PlaceTower),
}
