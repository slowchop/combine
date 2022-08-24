extern crate core;

pub mod game;
pub mod protocol;
pub mod ticks;

use crate::ticks::Ticks;
use naia_shared::{
    derive_channels, Channel, ChannelDirection, ChannelMode, LinkConditionerConfig,
    OrderedReliableReceiver, Protocolize, ReliableSettings, SharedConfig, SocketConfig,
    TickBufferSettings,
};
pub use protocol::auth::Auth;
pub use protocol::join_friend_game::JoinFriendGame;
pub use protocol::join_random_game::JoinRandomGame;
use rand::{thread_rng, Rng};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

pub const UDP_PORT: u16 = 24191;

pub const WEB_CONNECT_PORT: u16 = 24191; // Runs a web server for POST?
pub const WEB_PORT: u16 = 24192;

// pub const PROD_URL: &str = "http://45.248.51.162";
pub const PROD_URL: &str = "http://10.0.4.14";

pub const DEV_URL: &str = "http://10.0.4.14";

// 1000 / 20fps = 50ms
pub const MS_PER_TICK: u64 = 50;
pub const TICKS_PER_SECOND: u64 = 1000 / 50;

/// 0 is just after sunrise, so everything is bright.
/// 10 is when the sun starts setting.
/// 15 it gets dark -- Creeps released!
///
/// 28 "sunrise"
// pub const TICKS_PER_DAY: Ticks = Ticks(30 * TICKS_PER_SECOND as i64);
// pub const RESPAWN_CLOCK_TIME: Ticks = Ticks(20 * TICKS_PER_SECOND as i64);
// pub const RELEASE_CLOCK_TIME: Ticks = Ticks(15 * TICKS_PER_SECOND as i64);

pub const TICKS_PER_DAY: Ticks = Ticks(100 * TICKS_PER_SECOND as i64);
pub const RESPAWN_CLOCK_TIME: Ticks = Ticks(2 * TICKS_PER_SECOND as i64);
pub const RELEASE_CLOCK_TIME: Ticks = Ticks(4 * TICKS_PER_SECOND as i64);

pub fn shared_config() -> SharedConfig<Channels> {
    let tick_interval = Some(Duration::from_millis(MS_PER_TICK as u64));
    let link_condition = if false {
        Some(LinkConditionerConfig::average_condition())
    } else {
        None
    };

    // let link_condition = Some(LinkConditionerConfig {
    //     incoming_latency: 500,
    //     incoming_jitter: 100,
    //     incoming_loss: 0.01,
    // });

    SharedConfig::new(
        SocketConfig::new(link_condition, None),
        CHANNEL_CONFIG,
        tick_interval,
        None,
    )
}

#[derive(Debug, Clone)]
pub enum Env {
    Prod,
    Dev,
}

#[derive(Debug, Clone)]
pub struct Network {
    pub url: String,
    pub env: Env,
}

pub fn network_resource(debug: bool) -> Network {
    if debug {
        Network {
            url: DEV_URL.to_string(),
            env: Env::Dev,
        }
    } else {
        Network {
            env: Env::Prod,
            url: PROD_URL.to_string(),
        }
    }
}

#[derive(Copy)]
#[derive_channels]
pub enum Channels {
    PlayerCommand,
    ServerCommand,
    ServerUpdate,
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
    // This was dropping packets 100% of the time, maybe the rng was unlucky...
    // Channel {
    //     index: Channels::ServerUpdate,
    //     direction: ChannelDirection::ServerToClient,
    //     mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
    // },
];
