extern crate hyperspeed;

use hyperspeed::*;
use hyperspeed::core::{World, Engine, MasterController, EngineInstruction, ClientView};

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

// This is a demo of what Hyperspeed can do.
// It is a simple pong game, meant to be played by two players.

struct Position {
    pub x: f32,
    pub y: f32
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

struct MoveSystem {}

impl<'a> System<'a> for MoveSystem {
    type SystemData = WriteStorage<'a, Position>;

    fn run(&mut self, mut pos: Self::SystemData) {
        for mut p in (&mut pos).join() {
            p.x += 1.0;
        }
    }
}

struct RenderSystem {}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (WriteViewMap<'a>, Read<'a, bool>, ReadStorage<'a, Position>);
    fn run(&mut self, (mut view_map, should_render, positions): Self::SystemData) {
        if *should_render {
            for p in positions.join() {
                view_map.insert("default_key".to_string(), ClientView { sprites: vec!["asdf".to_string()], loc: vec![(p.x, p.y)] });
            }
        }
    }
}

struct MC {}

impl MasterController for MC {
    type ObserverEvent = Message;

    fn start(&mut self, world: &mut World, dt: f64) {
        let mut e = world.ecs_world.create_entity().with(Position { x: 100.0, y: 100.0}).build();
    }

    fn tick(&mut self, world: &mut World, dt: f64) -> EngineInstruction {
        // Regulate ticks
        sleep(Duration::from_millis(200));
        if world.connections.connections.len() > 0 {
            world.ecs_world.add_resource(true);
        } else {
            world.ecs_world.add_resource(false);
        }
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
    let mut engine = Engine::<Message>::new().with_mc(MC {})
        .with_system(MoveSystem {}, "m", &[])
        .with_system(RenderSystem {}, "render", &["m"])
        .build();
    if let Some(mut engine) = engine {
        engine.register::<Position>();
        engine.start_server();
        loop {
            engine.tick();
        }
    } else {
        println!("Engine could not be initialized!");
    }
}