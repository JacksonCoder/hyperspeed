#![allow(dead_code)]
#![feature(duration_float)]


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

pub use specs::prelude::{Write, Dispatcher, System, Component, VecStorage, Entities, WriteStorage, ReadStorage, Read, Entity, Join};

pub use ::core::*;

pub use self::utils::*;