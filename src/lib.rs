#![allow(dead_code)]

// Hyperspeed Engine is a fusion of an ECS and an observer-event system.
// For example, how would picking up an item work with an ECS?
// With Hyperspeed, it's easy: an Observer checks for a player clicking on an item and registers that
// they should pick up the item: this is then handled by either a controller or a system accessing the Events resource.

extern crate specs;

#[macro_use]
extern crate cascade;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bytes;

pub mod core;
pub mod utils;
pub mod systems;
pub mod components;

pub use specs::prelude::{Dispatcher, System, Component, VecStorage, Entities, WriteStorage, ReadStorage, Read, Entity, Join};

pub use ::core::*;

pub use self::utils::*;