use crate::net::ReleaseCreepEvent;
use bevy::prelude::*;

pub fn release_creeps(mut release_the_creeps_events: EventReader<ReleaseCreepEvent>) {
    todo!("This loop isnt getting anything");
    for release_the_creep_event in release_the_creeps_events.iter() {
        println!("Release the creeps! {:?}", release_the_creep_event);
    }
}
