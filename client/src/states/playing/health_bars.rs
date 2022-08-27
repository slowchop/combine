use crate::states::playing::hurt_entities::Damaged;
use bevy::prelude::*;
use shared::game::defs::{CreepRef, Defs};

#[derive(Component)]
pub struct HealthBar(Entity);

#[derive(Component)]
pub struct HasHealthBar;

/// Has damage but no health bar yet.
pub fn add_health_bars(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &Transform), (With<Damaged>, Without<HasHealthBar>)>,
) {
    for (entity, transform) in query.iter() {
        println!("No health bar, spawning");
        let mesh = meshes.add(shape::Quad::new(Vec2::new(1.0, 0.2)).into());
        let material = materials.add(StandardMaterial::from(Color::rgba(0.0, 1.0, 0.0, 0.0)));

        commands.entity(entity).insert(HasHealthBar);
        commands
            .spawn_bundle(MaterialMeshBundle {
                material,
                transform: transform.with_translation(Vec3::new(0.0, 0.5, 1.0)),
                mesh,
                ..Default::default()
            })
            .insert(HealthBar(entity));
    }
}

pub fn health_bars(
    mut commands: Commands,
    defs: Res<Defs>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    creep_query: Query<(
        &Damaged,
        &CreepRef,
        &mut Transform,
        &Handle<StandardMaterial>,
    )>,
    mut health_bar_query: Query<(Entity, &mut Transform, &HealthBar), Without<CreepRef>>,
) {
    for (health_bar_entity, mut health_bar_transform, health_bar) in health_bar_query.iter_mut() {
        let (damaged, creep_ref, &transform, material) =
            if let Ok(c) = creep_query.get(health_bar.0) {
                c
            } else {
                warn!(
                    "Entity not found in creep_query for health_bars: {:?} Despawning. It's cool.",
                    health_bar.0
                );
                commands.entity(health_bar_entity).despawn();
                continue;
            };

        let creep = if let Some(creep) = defs.creep(&creep_ref) {
            creep
        } else {
            warn!("Creep not found in defs: {:?}", creep_ref);
            continue;
        };

        // From green to red.
        let fraction = damaged.0 as f32 / creep.health as f32;
        materials.get_mut(material).unwrap().base_color = Color::rgb(1.0 - fraction, fraction, 0.0);
        health_bar_transform.scale.x = fraction;
    }
}
