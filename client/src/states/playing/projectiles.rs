use bevy::prelude::*;
use shared::game::shared_game::ServerEntityId;

/// It says instant, and it is on the server, but we animate here to make it more interesting.
#[derive(Component)]
pub struct InstantProjectile {
    pub tower: ServerEntityId,
    pub creep: ServerEntityId,
    pub speed: f32,
}

pub fn instant_projectiles(mut commands: Commands, mut query: Query<(Entity, &InstantProjectile)>) {
    // for (entity, projectile) in query.iter_mut() {
    //     commands.despawn(entity);
    // }
}
