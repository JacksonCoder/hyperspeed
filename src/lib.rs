#![allow(dead_code)]
#![feature(duration_float)]
/// Hyperspeed is a game framework that gives developers
/// a powerful and intuitive API to create multiplayer games
/// in Rust.

extern crate specs;
#[macro_use]
extern crate shred_derive;
#[macro_use]
extern crate specs_derive;

#[macro_use]
extern crate cascade;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate bytes;

pub mod core;
pub mod utils;
pub mod systems;
pub mod components;

pub use specs::prelude::*;

pub use utils::*;
