use crate::game::owner::Owner;
use naia_shared::serde::{BitReader, BitWrite, Serde, SerdeErr};
use rand::{thread_rng, Rng};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct SharedPlayer {
    pub name: PlayerName,
    pub owner: Owner,
    pub gold: u32,
    pub lives: u32,
}

impl SharedPlayer {
    pub fn new_waiting(name: PlayerName) -> Self {
        SharedPlayer {
            name,
            owner: Owner::waiting(),
            gold: 1000,
            lives: 20,
        }
    }
}

impl Serde for SharedPlayer {
    fn ser(&self, writer: &mut dyn BitWrite) {
        self.name.ser(writer);
        self.owner.ser(writer);
        self.gold.ser(writer);
        self.lives.ser(writer);
    }
    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        Ok(SharedPlayer {
            name: Serde::de(reader)?,
            owner: Serde::de(reader)?,
            gold: Serde::de(reader)?,
            lives: Serde::de(reader)?,
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct PlayerName(pub String);

impl PlayerName {
    pub fn new(c: &str) -> Self {
        let mut s = c.to_string();
        s = s.to_ascii_uppercase();

        if c.len() == 0 {
            PlayerName::random()
        } else if c.len() > 3 {
            PlayerName(s[0..3].to_string())
        } else {
            PlayerName(s)
        }
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        let mut s = String::new();
        for _ in 0..3 {
            s.push(rng.gen_range('A'..'Z'));
        }
        PlayerName::new(&s)
    }
}

impl Display for PlayerName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serde for PlayerName {
    fn ser(&self, writer: &mut dyn BitWrite) {
        self.0.ser(writer);
    }
    fn de(reader: &mut BitReader) -> Result<Self, SerdeErr> {
        Ok(PlayerName(Serde::de(reader)?))
    }
}
