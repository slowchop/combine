use crate::state::{PlayerQueue, State};
use bevy_ecs::prelude::Res;
use bevy_ecs::system::ResMut;
use naia_bevy_server::shared::BigMapKey;
use naia_bevy_server::Server;
use shared::{Channels, Protocol};

pub fn match_randoms(
    mut player_queue: ResMut<PlayerQueue>,
    mut server: Server<Protocol, Channels>,
) {
    loop {
        let (a, b) = match player_queue.pair() {
            None => return,
            Some(p) => p,
        };

        let room = server.make_room();
        let room_key = room.key();

        let mut a = server.user_mut(&a);
        a.enter_room(&room_key);

        let mut b = server.user_mut(&b);
        b.enter_room(&room_key);
        println!("Joined room");
    }
}
