#![allow(dead_code)]

// Star Engine is a fusion of an ECS and an observer-event system.
// For example, how would picking up an item work with an ECS?

extern crate specs;

pub mod core;

pub use specs::prelude::{Dispatcher, System, Component, VecStorage, Entities, WriteStorage, ReadStorage, Read, Entity, Join};

pub use ::core::*;