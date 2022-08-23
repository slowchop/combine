use crate::{MS_PER_TICK, TICKS_PER_SECOND};
use std::ops::{Add, AddAssign, Div, Sub, SubAssign};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct Ticks(pub i64);

impl Ticks {
    pub fn new(ticks: i64) -> Self {
        Ticks(ticks)
    }

    pub fn to_secs_f64(&self) -> f64 {
        self.0 as f64 / TICKS_PER_SECOND as f64
    }

    pub fn to_duration(&self) -> Option<Duration> {
        if self.0 < 0 {
            None
        } else {
            Some(Duration::from_secs_f64(
                self.0 as f64 / TICKS_PER_SECOND as f64,
            ))
        }
    }
}

impl From<Duration> for Ticks {
    fn from(duration: Duration) -> Self {
        Ticks(duration.as_millis() as i64 / MS_PER_TICK as i64)
    }
}

impl From<u64> for Ticks {
    fn from(t: u64) -> Self {
        Ticks(t as i64)
    }
}

impl From<i64> for Ticks {
    fn from(t: i64) -> Self {
        Ticks(t)
    }
}

impl From<i32> for Ticks {
    fn from(t: i32) -> Self {
        Ticks(t as i64)
    }
}

impl Add for Ticks {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Ticks(self.0 + other.0)
    }
}

impl AddAssign for Ticks {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
    }
}

impl Sub for Ticks {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Ticks(self.0 - other.0)
    }
}

impl SubAssign for Ticks {
    fn sub_assign(&mut self, other: Self) {
        self.0 -= other.0;
    }
}

impl Div for Ticks {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Ticks(self.0 / other.0)
    }
}