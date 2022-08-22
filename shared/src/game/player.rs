use rand::{thread_rng, Rng};
use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct SharedPlayer {
    pub name: PlayerName,
    pub gold: u32,
    pub lives: u32,
}

impl SharedPlayer {
    pub fn new(name: PlayerName) -> Self {
        SharedPlayer {
            name,
            gold: 1000,
            lives: 20,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PlayerName(String);

impl PlayerName {
    pub fn new(c: &str) -> Self {
        let mut s = c.to_string();
        s = s.to_ascii_uppercase();

        if c.len() != 3 {
            PlayerName("PLR".into())
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
