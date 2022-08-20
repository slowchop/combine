mod auth;
mod join_game;

pub use auth::Auth;
pub use join_game::JoinGame;
use naia_shared::{
    derive_channels, Channel, ChannelDirection, ChannelMode, LinkConditionerConfig, Protocolize,
    SharedConfig, SocketConfig, TickBufferSettings,
};
use std::time::Duration;

pub const UDP_PORT: u16 = 24191;
pub const WEB_PORT: u16 = 24192;

#[derive(Protocolize)]
pub enum Protocol {
    Auth(Auth),
    JoinGame(JoinGame),
}

pub fn shared_config() -> SharedConfig<Channels> {
    // Set tick rate to ~60 FPS
    let tick_interval = Some(Duration::from_millis(20));

    //  let link_condition = None;
    let link_condition = Some(LinkConditionerConfig::average_condition());
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
    // EntityAssignment,
}

pub const CHANNEL_CONFIG: &[Channel<Channels>] = &[
    Channel {
        index: Channels::PlayerCommand,
        direction: ChannelDirection::ClientToServer,
        mode: ChannelMode::TickBuffered(TickBufferSettings::default()),
    },
    // Channel {
    //     index: Channels::EntityAssignment,
    //     direction: ChannelDirection::ServerToClient,
    //     mode: ChannelMode::UnorderedReliable(ReliableSettings::default()),
    // },
];

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert!(true);
    }
}
