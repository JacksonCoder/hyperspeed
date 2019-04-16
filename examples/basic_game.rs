extern crate hyperspeed;

use hyperspeed::*;
use hyperspeed::core::{World, Engine, MasterController, EngineInstruction};

use std::collections::HashMap;

// This is a demo of what Hyperspeed can do.
// It is a simple pong game, meant to be played by two players.

struct Position {
    x: u32,
    y: u32
}

define_component!(Position);

struct PlayerControllable {
    player_key: String
}

define_component!(PlayerControllable);

fn start_game(world: &mut World) -> bool {
    if world.connections.size() < 2 {
        println!("We can't start the game yet!");
        return false;
    }

    true
}

struct MC {}

impl MasterController for MC {
    type ObserverEvent = Message;

    fn tick(&mut self, world: &mut World, dt: f64) -> EngineInstruction {
        EngineInstruction::Run {
            run_dispatcher: true
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Message {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let mut engine = Engine::<Message>::new().with_mc(MC {}).build();
    if let Some(mut engine) = engine {
        engine.start_server();
        loop {
            engine.tick();
        }
    } else {
        println!("Engine could not be initialized!");
    }
}