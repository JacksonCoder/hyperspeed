use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::thread::spawn;
use std::collections::{HashMap, VecDeque};
use std::time::Instant;
use std::net::TcpStream;

use specs::Component;

use super::*;
use super::server::InputBufferMutex;
use crate::utils::*;
use crate::components::{Position, Camera, Visible};
use crate::core::subsystem::{Subsystem, EngineInstruction};

pub struct Engine<'a, 'b, E: Sync + Send + Clone + 'static> {
    pub world: World<'a, 'b, E>,
    subsystems: Vec<Box<Subsystem<MessageType=E>>>,
    input_buffer: Option<InputBufferMutex>,
    server_conf: ServerConfig,
    prev_time: Instant,
    connection_channel: Receiver<(Connection, Sender<ClientView>)>,
    view_channels: HashMap<String, Sender<ClientView>>,
    server_stream_handler: Option<StreamHandler>
}

pub struct EngineBuilder<'a, 'b, E: Sync + Send + Clone + 'static> {
    server_conf: ServerConfig,
    system_executor_builder: SystemExecutorBuilder<'a, 'b>,
    subsystems: Vec<Box<Subsystem<MessageType=E>>>,
    server_stream_handler: Option<StreamHandler>
}

impl<'a, 'b, E: Sync + Send + Clone + 'static> Engine<'a, 'b, E> {
    pub fn new() -> EngineBuilder<'a, 'b, E> {
        EngineBuilder {
            server_conf: ServerConfig::new(),
            system_executor_builder: SystemExecutor::new(),
            subsystems: vec!(),
            server_stream_handler: None
        }
    }

    pub fn init_resources(&mut self) {
        self.world.init_resources();
    }

    pub fn register<T: Component>(&mut self)
    where <T as Component>::Storage : std::default::Default {
        self.world.register::<T>();
    }

    pub fn start_server(&mut self) {
        fn default(t: &mut TcpStream) -> StreamData {
            StreamData::do_connect_str("default_key")
        }


        let (sender, reciever) = channel();

        let mut handler = None;

        ::std::mem::swap(&mut self.server_stream_handler, &mut handler);

        let handler = handler
            .unwrap_or(default);

        let mut server = Server::new(self.server_conf.clone(), sender, handler);

        self.connection_channel = reciever;

        self.input_buffer = Some(server.get_input_buffer()); // Get a reference to the input buffer even after it gets moved to another thread

        spawn( move || server.main_loop());

        self.prev_time = Instant::now();

        // Initialize subsystems
        for ss in &mut self.subsystems {
            ss.start(&mut self.world);
        }
    }

    fn get_new_connection(&mut self) -> Option<(Connection, Sender<ClientView>)> {
        match self.connection_channel.try_recv() {
            Ok(C) => Some(C),
            Err(E) => match E {
                Empty => None,
                Disconnected => panic!("Engine fault: Server channel was disconnected")
            }
        }
    }

    fn get_inputs(&mut self) -> HashMap<String, VecDeque<Input>> {
        let mut lock = self.input_buffer.as_mut().unwrap().lock();
        match lock {
            Ok(ref mut lock) => {
                let mut input_map = HashMap::new();
                ::std::mem::swap(&mut input_map, lock);
                input_map
            }
            _ => {
                panic!("The input buffer mutex was poisoned!");
            }
        }

    }

    fn subsystems_tick(&mut self, delta_time: f64) -> EngineInstruction {
        let mut instruction = EngineInstruction::Run(RunData { run_dispatcher: true }, 0);
        for ss in &mut self.subsystems {
            instruction *= ss.tick(&mut self.world, delta_time);
        }
        instruction
    }
    
    pub fn tick(&mut self) {
        let tmp = self.prev_time;
        self.prev_time = Instant::now();
        let time = self.prev_time - tmp;

        let mut new_connection = self.get_new_connection();

        while new_connection.is_some() {
            println!("Processing new connection!");
            match new_connection.unwrap() {
                (conn, sender) => {
                    self.view_channels.insert(conn.key.clone(), sender);
                    self.world.add_connection(conn);
                }
            }
            new_connection = self.get_new_connection();
        }

        self.world.update_connections_in_world();

        let instruction = self.subsystems_tick(time.as_float_secs());

        match instruction {
            EngineInstruction::Run (
                RunData {
                    run_dispatcher
                },
                _
            ) => {
                if run_dispatcher {
                    // Clear world messages
                    self.world.get_world_mut().add_resource(Messages::<E>::new());

                    let inputs = self.get_inputs();
                    self.world.set_input(inputs);
                    self.world.tick();
                    self.world.maintain();
                }
            }
            EngineInstruction::Pause(_) => return,
            EngineInstruction::Restart(_) => unimplemented!()
        }

        let views = self.world.pop_views();
        // Send views through view channels
        for (key, view) in views {
            match self.view_channels.get_mut(&key) {
                Some(channel) => {
                    match channel.send(view) {
                            Ok(_) => {},
                            Err(_) => {
                            println!("Engine detects client stream thread has exited. Deleting connection.");
                            self.view_channels.remove(&key); // TODO: Remove connection
                            self.remove_connection(&key);
                        }
                    }
                },
                None => {
                    // The view channel does not exist, but it could be initialised later. So we do nothing here.
                }
            }
        }
    }

    fn remove_connection(&mut self, key: &String) {
        self.world.remove_connection(key);
    }
}

impl<'a, 'b, E: Sync + Send + Clone + 'static> EngineBuilder<'a, 'b, E> {
    pub fn with_name(mut self, name: &str) -> Self {
        self.server_conf.server_name = name.to_string();
        self
    }
    
    pub fn on_port(mut self, port: u16) -> Self {
        self.server_conf.port = port;
        self
    }
    
    pub fn with_system<S>(mut self, system: S, name: &str, dep: &[&str]) -> Self
    where
        S: for<'c> specs::System<'c> + Send + 'a {
        self.system_executor_builder.add_system(system, name, dep);
        self
    }
    
    pub fn with_subsystem<S: 'static>(mut self, subsystem: S) -> Self
    where
        S: Subsystem<MessageType=E> {
        self.subsystems.push(Box::new(subsystem));
        self
    }
    pub fn with_stream_handler(mut self, handler: StreamHandler) -> Self {
        self.server_stream_handler = Some(handler);
        self
    }
    
    pub fn build(mut self) -> Option<Engine<'a, 'b, E>> {
        let mut engine = Engine {
            world: World::new(self.system_executor_builder),
            subsystems: self.subsystems,
            server_conf: self.server_conf,
            input_buffer: None,
            prev_time: Instant::now(),
            // This is a fake channel
            connection_channel: channel().1,
            view_channels: HashMap::new(),
            server_stream_handler: self.server_stream_handler
        };
        engine.init_resources();
        Some(engine)
    }
}