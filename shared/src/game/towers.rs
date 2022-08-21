use bevy_ecs::prelude::*;
use strum::FromRepr;

#[derive(Component, Debug, FromRepr)]
pub enum Tower {
    MachineGun,
    Laser,
    FlameThrower,
    Cold,
    Electric,
    Missile,

    BossFire,
    BossCold,
    BossLaser,

    ComboBossTower,
}
