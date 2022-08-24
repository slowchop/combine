use bevy_ecs::prelude::*;
use bevy_log::info;
use bevy_time::Time;
use naia_bevy_server::Server;
use shared::protocol::Protocol;
use shared::Channels;

pub fn tick(
    time: Res<Time>,
    mut last_time: Local<f64>,
    // mut global: ResMut<State>,
    mut server: Server<Protocol, Channels>,
) {
    let s = time.seconds_since_startup();
    if s - *last_time > 60.0 {
        info!(?s, ticks = ?server.server_tick(), "tick");
        *last_time = s;
    }

    // Update scopes of entities
    for (_, user_key, entity) in server.scope_checks() {
        // You'd normally do whatever checks you need to in here..
        // to determine whether each Entity should be in scope or not.

        // This indicates the Entity should be in this scope.
        server.user_scope(&user_key).include(&entity);

        // And call this if Entity should NOT be in this scope.
        // server.user_scope(..).exclude(..);
    }

    // Process all received commands
    // for (entity, last_command) in global.player_last_command.drain() {
    //     if let Ok(mut position) = position_query.get_mut(entity) {
    //         shared_behavior::process_command(&last_command, &mut position);
    //     }
    // }

    // This is very important! Need to call this to actually send all update packets
    // to all connected Clients!
    server.send_all_updates();
}
