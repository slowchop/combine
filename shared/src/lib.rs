mod auth;
mod join_game;
pub mod player_name;
mod random_game;

pub use crate::random_game::JoinRandomGame;
pub use auth::Auth;
pub use join_game::JoinFriendGame;
use naia_shared::{
    derive_channels, Channel, ChannelDirection, ChannelMode, LinkConditionerConfig,
    OrderedReliableReceiver, Protocolize, ReliableSettings, SharedConfig, SocketConfig,
    TickBufferSettings,
};
use std::time::Duration;

pub const UDP_PORT: u16 = 24191;
pub const WEB_PORT: u16 = 24192;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    JoinRandomGame(JoinRandomGame),
    JoinFriendGame(JoinFriendGame),
}

pub fn shared_config() -> SharedConfig<Channels> {
    let tick_interval = Some(Duration::from_millis(50)); // 1000 / 20fps = 50ms

    let link_condition = None;
    // let link_condition = Some(LinkConditionerConfig::average_condition());
    //  let link_condition = Some(LinkConditionerConfig {
    //      incoming_latency: 500,
    //      incoming_jitter: 1,
    //      incoming_loss: 0.0,
    //  });
    SharedConfig::new(
        SocketConfig::new(link_condition, None),
        CHANNEL_CONFIG,
        tick_interval,
        None,
    )
}

#[derive_channels]
pub enum Channels {
    PlayerCommand,
    ServerCommand,
}

pub const CHANNEL_CONFIG: &[Channel<Channels>] = &[
    Channel {
        index: Channels::PlayerCommand,
        direction: ChannelDirection::ClientToServer,
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
    },
    Channel {
        index: Channels::ServerCommand,
        direction: ChannelDirection::ServerToClient,
        mode: ChannelMode::OrderedReliable(ReliableSettings::default()),
    },
];
