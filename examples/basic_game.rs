extern crate hyperspeed;

use hyperspeed::{System, WriteStorage, ReadStorage, Read, WriteViewMap, Entities, WriteConnections, define_component, Component, VecStorage, Join, ReadInputMap, WriteMessages, ReadMessages};
use hyperspeed::core::{World, Engine, EngineInstruction, ClientView, StreamData, Subsystem, RunData, key_pressed_for};

use std::thread::sleep;
use std::time::Duration;
use std::net::TcpStream;
use bytes::{BufMut, BytesMut};
use std::io::Read as R;
use specs::world::EntitiesRes;
use hyperspeed::utils::server::read_message_from_stream;
use hyperspeed::utils::server::StreamReadResult::{ValidMessage, StreamError, InvalidMessage};
use hyperspeed::components::Visible;
use specs::join::JoinIter;


struct Position {
    pub x: f32,
    pub y: f32
}

define_component!(Position);

struct PlayerControllable {
    pub player_key: String
}

define_component!(PlayerControllable);

fn start_game(world: &mut World<Message>) -> bool {
    if world.get_connections().size() < 2 {
        println!("We can't start the game yet!");
        return false;
    }

    true
}

struct MoveSystem {}

impl<'a> System<'a> for MoveSystem {
    type SystemData = (ReadStorage<'a, PlayerControllable>, WriteStorage<'a, Position>, ReadMessages<'a, Message>);

    fn run(&mut self, (players, mut pos, messages): Self::SystemData) {
        use Message::*;
        for message in &*messages {
            match message {
                Up(ref key) => {
                    for (player, pos) in (&players, &mut pos).join() {
                        if player.player_key == *key {
                            pos.y += 0.1;
                        }
                    }
                }
                Down(ref key) => {
                    for (player, pos) in (&players, &mut pos).join() {
                        if player.player_key == *key {
                            pos.y -= 0.1;
                        }
                    }
                }
                Left(ref key) => {
                    for (player, pos) in (&players, &mut pos).join() {
                        if player.player_key == *key {
                            pos.x -= 0.1;
                        }
                    }
                }
                Right(ref key) => {
                    for (player, pos) in (&players, &mut pos).join() {
                        if player.player_key == *key {
                            pos.x += 0.1;
                        }
                    }
                }
            }
        }
    }
}

struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (ReadInputMap<'a>, WriteMessages<'a, Message>);

    fn run(&mut self, (input, mut messages): Self::SystemData) {
        // Generate moves for players.
        for key in input.keys() {
            if key_pressed_for(&input, key, 'w') {
                messages.push(Message::Up(key.clone()));
            }
            if key_pressed_for(&input, key, 'a') {
                messages.push(Message::Left(key.clone()));
            }
            if key_pressed_for(&input, key, 's') {
                messages.push(Message::Down(key.clone()));
            }
            if key_pressed_for(&input, key, 'd') {
                messages.push(Message::Right(key.clone()));
            }
        }
    }
}

struct ConnectionSystem {}

impl<'a> System<'a> for ConnectionSystem {
    type SystemData = (Entities<'a>, WriteConnections<'a>, WriteStorage<'a, Position>, WriteStorage<'a, PlayerControllable>, WriteStorage<'a, Visible>);

    fn run(&mut self, (entities, mut connections, mut pos, mut player_controllable, mut visible): Self::SystemData) {
        for key in (*connections).pop_new_keys() {
            println!("Making new entity!!");
            entities.build_entity()
                .with(PlayerControllable { player_key: key }, &mut player_controllable)
                .with(Position { x: 100.0, y: 100.0 }, &mut pos)
                .with(Visible { sprite: 0 }, &mut visible)
                .build();
        }
    }
}

struct RenderSystem {}

impl<'a> System<'a> for RenderSystem {
    type SystemData = (WriteViewMap<'a>, Read<'a, bool>, ReadStorage<'a, Position>, ReadStorage<'a, PlayerControllable>);
    fn run(&mut self, (mut view_map, should_render, positions, players): Self::SystemData) {
        if *should_render {
            for pc in players.join() {
                let mut view = ClientView {
                    sprites: vec!(),
                    loc: vec!()
                };
                for p in positions.join() {
                    view.sprites.push(0);
                    view.loc.push((p.x, p.y));
                }
                view_map.insert(pc.player_key.clone(), view);
            }
        }
    }
}

struct SS {}

impl Subsystem for SS {
    type MessageType = Message;

    fn tick(&mut self, world: &mut World<Self::MessageType>, dt: f64) -> EngineInstruction {
        // Regulate ticks
        sleep(Duration::from_millis(20));
        if world.get_connections().size() > 0 {
            world.get_world_mut().add_resource(true);
        } else {
            world.get_world_mut().add_resource(false);
        }
        EngineInstruction::Run (
            RunData {
                run_dispatcher: true
            },
            1
        )
    }
}

fn process_stream(mut stream: &mut TcpStream) -> StreamData {
    let mut buffer = BytesMut::new();
    buffer.reserve(512);
    buffer.put(&[0; 512][..]);
    match read_message_from_stream(&mut stream, &mut buffer) {
        ValidMessage(msg) => StreamData::do_connect(msg),
        InvalidMessage => StreamData::dont_connect(),
        StreamError(e) => StreamData::dont_connect(),
        _ => unreachable!()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Message {
    Up(String),
    Down(String),
    Left(String),
    Right(String)
}

fn main() {
    let mut engine = Engine::<Message>::new().with_subsystem(SS {})
        .with_system(ConnectionSystem {}, "c", &[])
        .with_system(InputSystem {}, "i", &[])
        .with_system(MoveSystem {}, "m", &["c", "i"])
        .with_system(RenderSystem {}, "render", &["m"])
        .with_stream_handler(process_stream)
        .build();
    if let Some(mut engine) = engine {
        engine.register::<Position>();
        engine.register::<PlayerControllable>();
        engine.start_server();
        loop {
            engine.tick();
        }
    } else {
        println!("Engine could not be initialized!");
    }
}