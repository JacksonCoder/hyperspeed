extern crate hyperspeed;

use hyperspeed::*;
use hyperspeed::core::*;

use std::collections::HashMap;

struct Position {
    x: i32,
    y: i32
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

struct Velocity {
    x: i32,
    y: i32
}

struct Item {
    inside: Option<Entity>
}

impl Component for Item {
    type Storage = VecStorage<Self>;
}

struct Player {
    items: Vec<Entity>,
    key: String
}

impl Component for Player {
    type Storage = VecStorage<Self>;
}


struct MotionSystem;

struct PickupController;

#[derive(Clone)]
enum Event {
    Empty,
    ItemPickup {
        item_id: Entity,
        player_id: Entity
    }
}

impl Default for Event {
    fn default() -> Event {
       Event::Empty
    }
}

struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (WriteStorage<'a, Player>, ReadStorage<'a, Position>, Read<'a, Events<Event>>);
    
    fn run(&mut self, (mut players, pos, events): Self::SystemData) {
        for (player, pos) in (&mut players, &pos).join() {
            // TODO: impl
        }
    }
}

struct M;

impl MasterController for M {
    type ObserverEvent = Event;
    
    fn tick(&mut self, world: &mut World, delta_time: f32) -> EngineInstruction {
        EngineInstruction::Run {
            run_dispatcher: true
        }
    }
}

fn main() {
    let mut engine = Engine::new().with_mc(M {})
                                  .build().unwrap();
    
    engine.start_server();

    loop {
        engine.tick();
    }
}