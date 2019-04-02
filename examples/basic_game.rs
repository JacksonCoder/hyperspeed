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

impl Controller for PickupController {
    type ObserverEvent = Event;
    fn event(&mut self, world: &mut World, e: &Self::ObserverEvent) {
        match e {
            Event::ItemPickup { item_id, player_id } => {
                let mut write_item = world.ecs_world.write_storage::<Item>();
                let mut write_player = world.ecs_world.write_storage::<Player>();
                let mut maybe_item = write_item.get_mut(*item_id);
                let mut maybe_player = write_player.get_mut(*player_id);
                if let Some(item) = maybe_item {
                    if let Some(player) = maybe_player {
                        item.inside = Some(player_id.clone());
                        player.items.push(item_id.clone());
                    }
                }
            }
            _ => {}
        }
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

struct O;

impl Observer for O {
    type ObserverEvent = Event;
}

struct M;

impl MasterController for M {
    type ObserverEvent = Event;
    
    fn tick(&mut self, world: &mut World, delta_time: f32) -> EngineInstruction {
        EngineInstruction::Run {
            run_controllers: true,
            run_observer: true,
            run_systems: true
        }
    }
}

fn main() {
    let mut engine = Engine::new().with_observer(O {})
                                  .with_mc(M {})
                                  .build().unwrap();
    
    engine.tick();
}