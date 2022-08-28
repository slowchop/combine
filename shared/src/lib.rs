extern crate core;

pub mod game;
pub mod protocol;
pub mod ticks;

use crate::ticks::Ticks;
use bevy_render::color::Color;
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

#[cfg(all(feature = "dev", feature = "prod"))]
compile_error!("Pick env");

#[cfg(all(not(feature = "dev"), not(feature = "prod")))]
compile_error!("Pick one env");

pub const SESSION_LISTEN_PORT: u16 = 24191;
pub const WEBRTC_LISTEN_ADDR: u16 = 24192; // Runs a web server for POST?
pub const PUBLIC_WEBRTC_PORT: u16 = 24192;

#[cfg(not(feature = "dev"))]
pub const URL: &str = "http://45.248.51.162";

#[cfg(feature = "dev")]
pub const URL: &str = "http://10.0.4.14";

pub const MS_PER_TICK: u64 = 250;
pub const TICKS_PER_SECOND: u64 = 1000 / MS_PER_TICK;

pub const TICKS_PER_DAY: Ticks = Ticks(60 * TICKS_PER_SECOND as i64);
pub const RESPAWN_CLOCK_TIME: Ticks = Ticks(1 * TICKS_PER_SECOND as i64);
pub const RELEASE_CLOCK_TIME: Ticks = Ticks(31 * TICKS_PER_SECOND as i64);

pub fn shared_config() -> SharedConfig<Channels> {
    let tick_interval = Some(Duration::from_millis(MS_PER_TICK as u64));
    let link_condition = None;
    // let link_condition = Some(LinkConditionerConfig::poor_condition());

    SharedConfig::new(
        SocketConfig::new(link_condition, None),
        CHANNEL_CONFIG,
        tick_interval,
        None,
    )
}

#[derive(Copy)]
#[derive_channels]
pub enum Channels {
    PlayerCommand,
    ServerCommand,
    ServerUnreliable,
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
    Channel {
        index: Channels::ServerUnreliable,
        direction: ChannelDirection::ServerToClient,
        mode: ChannelMode::UnorderedUnreliable,
    },
];
