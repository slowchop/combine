extern crate core;

pub mod game;
pub mod protocol;

use naia_shared::{
    derive_channels, Channel, ChannelDirection, ChannelMode, LinkConditionerConfig,
    OrderedReliableReceiver, Protocolize, ReliableSettings, SharedConfig, SocketConfig,
    TickBufferSettings,
};
pub use protocol::auth::Auth;
pub use protocol::join_friend_game::JoinFriendGame;
pub use protocol::join_random_game::JoinRandomGame;
use rand::{thread_rng, Rng};
use std::time::Duration;

pub const UDP_PORT: u16 = 24191;
pub const WEB_PORT: u16 = 24192;

// 1000 / 20fps = 50ms
pub const MS_PER_FRAME: u64 = 50;

pub fn shared_config() -> SharedConfig<Channels> {
    let tick_interval = Some(Duration::from_millis(MS_PER_FRAME as u64));

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

pub fn seen_hack() -> u64 {
    thread_rng().gen()
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

pub struct Ticks(pub u64);

impl Ticks {
    pub fn new(ticks: u64) -> Self {
        Ticks(ticks)
    }
}

impl From<Duration> for Ticks {
    fn from(duration: Duration) -> Self {
        Ticks(duration.as_millis() as u64 / MS_PER_FRAME)
    }
}
