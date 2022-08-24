use crate::game::owner::Owner;
use crate::game::shared_game::ServerEntityId;
use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};
use bevy_transform::prelude::Transform;
use bevy_utils::HashMap;
use naia_shared::serde::{BitReader, BitWrite, Serde, SerdeErr};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use strum::FromRepr;
use tracing::info;

pub const PIXELS_PER_METER: f32 = 250.;

#[derive(Serialize, Deserialize)]
pub struct Defs {
    pub levels: HashMap<String, LevelDef>,
    pub towers: HashMap<String, Tower>,
    pub creeps: HashMap<String, Creep>,
    pub textures: HashMap<String, TextureDefinition>,
}

impl Defs {
    pub fn load() -> Self {
        let data = include_str!("../../../game/assets/defs.yaml");
        serde_yaml::from_str(data).unwrap()
    }

    // This has to be run from the game directory!
    pub fn save(&self) {
        let yaml = serde_yaml::to_string(&self).unwrap();
        let bytes = yaml.as_bytes();

        File::create("assets/defs.yaml")
            .unwrap()
            .write_all(bytes)
            .unwrap();
    }

    pub fn level_entity_transform(
        &self,
        texture: &Option<String>,
        position: &Option<Vec2>,
    ) -> Option<Transform> {
        let texture_def = texture
            .as_ref()
            .and_then(|texture| self.textures.get(texture.as_str()))?;

        info!("{:?}", texture_def);

        let position = position.as_ref()?;
        info!("{:?}", position);
        let x = position.x;
        let y = position.y;
        Some(Transform::from_xyz(x, 0., y).with_scale(Vec3::new(
            texture_def.size[0] as f32 / PIXELS_PER_METER,
            texture_def.size[1] as f32 / PIXELS_PER_METER,
            1.0,
        )))
    }

    pub fn tower(&self, name: &str) -> Option<Tower> {
        self.towers.get(name).cloned()
    }

    pub fn creep(&self, name: &str) -> Option<Creep> {
        self.creeps.get(name).cloned()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tower {
    pub name: String,
    pub combo: Vec<String>,
    pub texture: String,
    pub damage: f32,
    pub range: f32,
    pub cost: u32,
}

#[derive(Component)]
pub struct TowerRef(pub String);

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Creep {
    pub name: String,
    pub combo: Vec<String>,
    pub texture: String,
    pub speed: f32,
    pub cost: u32,
}

#[derive(Component)]
pub struct CreepRef(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureDefinition {
    pub size: Vec2,
}

#[derive(Serialize, Deserialize)]
pub struct LevelDef {
    pub name: String,
    pub entities: Vec<EntityDef>,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EntityDef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub texture: Option<String>,
    #[serde(default, rename = "type")]
    pub entity_type: EntityType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<NetVec2>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<Owner>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<NetVec2>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tower: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creep: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_entity_id: Option<ServerEntityId>,
}

impl Serde for EntityDef {
    fn ser(&self, writer: &mut dyn BitWrite) {
        self.texture.ser(writer);
        self.entity_type.ser(writer);
        self.position.ser(writer);
        self.owner.ser(writer);
        self.radius.ser(writer);
        self.path.ser(writer);
        self.tower.ser(writer);
        self.creep.ser(writer);
        self.server_entity_id.ser(writer);
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        let texture: Option<String> = Serde::de(reader)?;
        let entity_type: EntityType = Serde::de(reader)?;
        let position: Option<NetVec2> = Serde::de(reader)?;
        let owner: Option<Owner> = Serde::de(reader)?;
        let radius: Option<f32> = Serde::de(reader)?;
        let path: Option<Vec<NetVec2>> = Serde::de(reader)?;
        let tower: Option<String> = Serde::de(reader)?;
        let creep: Option<String> = Serde::de(reader)?;
        let server_entity_id: Option<ServerEntityId> = Serde::de(reader)?;
        Ok(EntityDef {
            texture,
            entity_type,
            position,
            owner,
            radius,
            path,
            tower,
            creep,
            server_entity_id,
        })
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NetVec2(pub Vec2);

impl Serde for NetVec2 {
    fn ser(&self, writer: &mut dyn BitWrite) {
        self.0.x.ser(writer);
        self.0.y.ser(writer);
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        let x: f32 = Serde::de(reader)?;
        let y: f32 = Serde::de(reader)?;
        Ok(NetVec2(Vec2::new(x, y)))
    }
}

impl From<Vec2> for NetVec2 {
    fn from(v: Vec2) -> Self {
        NetVec2(v)
    }
}

impl From<&NetVec2> for Vec2 {
    fn from(nv: &NetVec2) -> Self {
        nv.0
    }
}

impl From<NetVec2> for Vec2 {
    fn from(nv: NetVec2) -> Self {
        nv.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct NetVec3(pub Vec3);

impl Serde for NetVec3 {
    fn ser(&self, writer: &mut dyn BitWrite) {
        self.0.x.ser(writer);
        self.0.y.ser(writer);
        self.0.z.ser(writer);
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        let x: f32 = Serde::de(reader)?;
        let y: f32 = Serde::de(reader)?;
        let z: f32 = Serde::de(reader)?;
        Ok(NetVec3(Vec3::new(x, y, z)))
    }
}

impl From<Vec3> for NetVec3 {
    fn from(v: Vec3) -> Self {
        NetVec3(v)
    }
}

impl From<&NetVec3> for Vec3 {
    fn from(nv: &NetVec3) -> Self {
        nv.0
    }
}

impl From<NetVec3> for Vec3 {
    fn from(nv: NetVec3) -> Self {
        nv.0
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Eq, PartialEq, FromRepr)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum EntityType {
    Sprite,
    Ground,
    Spawn,
    Base,
    Path,
    Tower,
    Creep,
}

impl Default for EntityType {
    fn default() -> Self {
        EntityType::Sprite
    }
}

impl Serde for EntityType {
    fn ser(&self, writer: &mut dyn BitWrite) {
        writer.write_byte(*self as u8);
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        EntityType::from_repr(reader.read_byte()).ok_or(SerdeErr {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use naia_shared::serde::BitWriter;

    #[test]
    fn test_entity_def() {
        let mut writer = BitWriter::new();
        let entity_def_1 = EntityDef {
            texture: Some("texture".to_string()),
            entity_type: EntityType::Tower,
            position: Some(Vec2::new(1.0, 2.0).into()),
            owner: Some(Owner::new(0)),
            radius: Some(3.0),
            path: Some(vec![Vec2::new(1.0, 2.0).into(), Vec2::new(3.0, 4.0).into()]),
            tower: Some("tower".to_string()),
            creep: None,
            server_entity_id: Some(ServerEntityId(1)),
        };

        entity_def_1.ser(&mut writer);
        let (buffer_length, buffer) = writer.flush();

        dbg!(buffer_length, buffer);

        let mut reader = BitReader::new(&buffer[..buffer_length]);
        let entity_def_2 = EntityDef::de(&mut reader).unwrap();

        assert_eq!(entity_def_1, entity_def_2);
    }
}
