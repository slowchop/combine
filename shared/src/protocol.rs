use crate::protocol::game_ready::GameReady;
use crate::protocol::net_position::NetPosition;
use crate::protocol::release_the_creeps::ReleaseCreep;
use crate::protocol::request_tower_placement::RequestTowerPlacement;
use crate::protocol::spawn_entity::SpawnEntity;
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
pub mod net_position;
pub mod release_the_creeps;
pub mod request_tower_placement;
pub mod spawn_entity;

#[derive(Protocolize)]
pub enum Protocol {
    SpawnEntity(SpawnEntity),
    NetPosition(NetPosition),
    ReleaseCreep(ReleaseCreep),
    Auth(Auth),
    JoinRandomGame(JoinRandomGame),
    JoinFriendGame(JoinFriendGame),
    GameReady(GameReady),
    RequestTowerPlacement(RequestTowerPlacement),
}
