use crate::protocol::destroy_entity::DestroyEntity;
use crate::protocol::game_ready::GameReady;
use crate::protocol::release_creep::ReleaseCreeps;
use crate::protocol::request_tower_placement::RequestTowerPlacement;
use crate::protocol::spawn_entity::SpawnEntity;
use crate::protocol::update_position::UpdatePosition;
use crate::{Auth, JoinFriendGame};
pub use join_random_game::JoinRandomGame;
use naia_shared::{
    Channel, ChannelDirection, ChannelMode, LinkConditionerConfig, OrderedReliableReceiver,
    Protocolize, ReliableSettings, SharedConfig, SocketConfig, TickBufferSettings,
};

pub mod auth;
pub mod destroy_entity;
pub mod game_ready;
pub mod join_friend_game;
pub mod join_random_game;
pub mod release_creep;
pub mod request_tower_placement;
pub mod spawn_entity;
pub mod update_position;

#[derive(Protocolize)]
pub enum Protocol {
    UpdatePosition(UpdatePosition),
    DestroyEntity(DestroyEntity),
    SpawnEntity(SpawnEntity),
    ReleaseCreeps(ReleaseCreeps),
    Auth(Auth),
    JoinRandomGame(JoinRandomGame),
    JoinFriendGame(JoinFriendGame),
    GameReady(GameReady),
    RequestTowerPlacement(RequestTowerPlacement),
}
