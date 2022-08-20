use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, bevy::reflect::TypeUuid)]
#[uuid = "3d95a211-1b29-44a3-a9db-875cf44ff92c"]
pub struct Level {
    pub name: String,
    // pub actors: Actor,
}
