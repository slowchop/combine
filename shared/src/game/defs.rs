use crate::game::components::Tooltip;
use crate::game::owner::Owner;
use crate::game::shared_game::ServerEntityId;
use bevy_ecs::prelude::Component;
use bevy_math::{Vec2, Vec3};
use bevy_transform::prelude::Transform;
use bevy_utils::{HashMap, HashSet};
use naia_shared::serde::{BitReader, BitWrite, Serde, SerdeErr};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use strum::FromRepr;
use tracing::info;

pub const PIXELS_PER_METER: f32 = 250.;

#[derive(Serialize, Deserialize)]
pub struct Defs {
    pub levels: HashMap<String, LevelDef>,
    pub towers: HashMap<TowerRef, Tower>,
    pub creeps: HashMap<CreepRef, Creep>,
    pub textures: HashMap<String, TextureDefinition>,

    #[serde(skip)]
    pub tower_costs: Option<HashMap<TowerRef, (u32, u32)>>,
}

impl Defs {
    pub fn load() -> Self {
        let data = include_str!("../../../client/assets/defs.yaml");
        let mut defs: Defs = serde_yaml::from_str(data).unwrap();

        let mut tower_costs = HashMap::new();
        for tower_ref in defs.towers.keys() {
            let (cost, base_towers) = defs.calc_total_tower_cost(tower_ref);
            tower_costs.insert(tower_ref.clone(), (cost, base_towers));
        }
        defs.tower_costs = Some(tower_costs);

        defs
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

        let position = position.as_ref()?;
        let x = position.x;
        let y = position.y;
        Some(Transform::from_xyz(x, 0., y).with_scale(Vec3::new(
            texture_def.size[0] as f32 / PIXELS_PER_METER,
            texture_def.size[1] as f32 / PIXELS_PER_METER,
            1.0,
        )))
    }

    pub fn tower(&self, tower_ref: &TowerRef) -> Option<Tower> {
        self.towers.get(tower_ref).cloned()
    }

    pub fn tower_for_combo(&self, combo: &[&TowerRef]) -> Option<Tower> {
        let mut combo = combo.iter().map(|c| *c).collect::<HashSet<&TowerRef>>();

        for tower in self.towers.values() {
            let tower_combo = tower
                .combo
                .iter()
                .map(|c| c)
                .collect::<HashSet<&TowerRef>>();
            if combo == tower_combo {
                return Some(tower.clone());
            }
        }
        None
    }

    // Cost in base towers and in $
    pub fn calc_total_tower_cost(&self, tower_ref: &TowerRef) -> ((u32, u32)) {
        let tower = self.towers.get(tower_ref).unwrap();
        let mut cost = tower.cost;
        let mut base_towers = 0;
        if tower.combo.len() == 0 {
            base_towers += 1;
        } else {
            for parent in &tower.combo {
                let (this_cost, this_base_towers) = self.calc_total_tower_cost(parent);
                cost += this_cost;
                base_towers += this_base_towers;
            }
        }
        (cost, base_towers)
    }

    pub fn creep(&self, creep_ref: &CreepRef) -> Option<Creep> {
        self.creeps.get(creep_ref).cloned()
    }

    pub fn creep_for_combo(&self, combo: &[&CreepRef]) -> Option<Creep> {
        let combo = combo.iter().map(|c| *c).collect::<HashSet<&CreepRef>>();

        for creep in self.creeps.values() {
            let creep_combo = creep
                .combo
                .iter()
                .map(|c| c)
                .collect::<HashSet<&CreepRef>>();
            if combo == creep_combo {
                return Some(creep.clone());
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub enum DamageType {
    MachineGun,
    Cold,
    Fire,
    Electric,
    Missile,
}

// #[derive(Component, Debug, Clone)]
// pub struct DamageOverTimeState {
//     pub damage: u32,
//     pub last_trigger: Duration,
//     pub end_at: Duration,
// }
//
// #[derive(Component, Debug, Clone)]
// pub struct DamageOverTime {
//     pub damage: u32,
//     pub interval: f32,
//     pub total_duration: f32,
//     pub damage_type: DamageType,
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tower {
    pub emoji: String,
    pub name: TowerRef,
    pub title: String,
    pub combo: Vec<TowerRef>,
    pub texture: String,
    pub range: f32,
    pub size: f32,
    pub cost: u32,
    pub reload: f32,
    pub instant_damage: u32,

    /// How much to slow down creeps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cold_slowdown_amount: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cold_slowdown_duration: Option<f32>,

    /// Use instant damage as the interval for damage over time
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_damage_seconds: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fire_damage_interval: Option<f32>,

    /// If this is set, this is an area effect applying to many creeps,
    /// otherwise it only attacks one creep.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area: Option<f32>,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct TowerRef(pub String);

impl Serde for TowerRef {
    fn ser(&self, writer: &mut dyn BitWrite) {
        self.0.ser(writer);
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        Ok(TowerRef(Serde::de(reader)?))
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Creep {
    pub name: CreepRef,
    pub title: String,
    pub combo: Vec<CreepRef>,
    pub texture: String,
    pub speed: f32,
    pub cost: u32,
    pub health: u32,
    pub size: f32,
}

#[derive(Component, Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct CreepRef(pub String);

impl Serde for CreepRef {
    fn ser(&self, writer: &mut dyn BitWrite) {
        self.0.ser(writer);
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        Ok(CreepRef(Serde::de(reader)?))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextureDefinition {
    pub size: Vec2,
}

#[derive(Serialize, Deserialize, Default)]
pub struct LevelDef {
    pub name: String,
    pub entities: Vec<EntityDef>,
}

impl LevelDef {
    pub fn get_path(&mut self, owner: Owner) -> &mut EntityDef {
        self.entities
            .iter_mut()
            .find(|e| {
                if e.entity_type != EntityType::Path {
                    return false;
                }

                if e.owner != Some(owner) {
                    return false;
                }

                return true;
            })
            .unwrap()
    }

    pub fn can_build_here(&self, owner: Owner, requested_position: &Vec2) -> bool {
        self.entities
            .iter()
            .find(|e| {
                if e.entity_type != EntityType::BuildableCircle {
                    return false;
                }
                if e.owner != Some(owner) {
                    return false;
                }

                let position: Vec2 = e.position.as_ref().unwrap().into();
                let radius = e.radius.unwrap();
                position.distance(*requested_position) < radius
            })
            .is_some()
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, Eq, Component)]
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
    pub tower: Option<TowerRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creep: Option<CreepRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_entity_id: Option<ServerEntityId>,

    #[serde(skip)]
    pub tooltip: Option<Tooltip>,
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
        // Intentionally skipping tooltip
    }

    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        let texture: Option<String> = Serde::de(reader)?;
        let entity_type: EntityType = Serde::de(reader)?;
        let position: Option<NetVec2> = Serde::de(reader)?;
        let owner: Option<Owner> = Serde::de(reader)?;
        let radius: Option<f32> = Serde::de(reader)?;
        let path: Option<Vec<NetVec2>> = Serde::de(reader)?;
        let tower: Option<TowerRef> = Serde::de(reader)?;
        let creep: Option<CreepRef> = Serde::de(reader)?;
        let server_entity_id: Option<ServerEntityId> = Serde::de(reader)?;
        // Intentionally skipping tooltip

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
            tooltip: None,
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

impl From<&Vec2> for NetVec2 {
    fn from(v: &Vec2) -> Self {
        NetVec2(v.clone())
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
    Guide,
    BuildableCircle,
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
