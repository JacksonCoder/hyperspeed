use super::world::*;
use super::Server;
use super::ServerConfig;
use super::PlayerInputBuffer;
use crate::utils::*;

use std::sync::{Arc, Mutex};
use std::thread::spawn;

use std::collections::{HashMap, VecDeque};
use specs::Component;
use super::server::InputBufferMutex;

pub struct Engine<'a, 'b, E: Sync + Send + Clone + 'static> {
    pub world: World<'a, 'b>,
    master_controller: Box<MasterController<ObserverEvent=E>>,
    input_buffer: Option<InputBufferMutex>,
    server_conf: ServerConfig
}

pub struct EngineBuilder<'a, 'b, E: Sync + Send + Clone + 'static> {
    server_conf: ServerConfig,
    system_executor_builder: SystemExecutorBuilder<'a, 'b>,
    master_controller: Option<Box<MasterController<ObserverEvent=E>>>,
}

impl<'a, 'b, E: Sync + Send + Clone + 'static> Engine<'a, 'b, E> {
    pub fn new() -> EngineBuilder<'a, 'b, E> {
        EngineBuilder {
            server_conf: ServerConfig::new(),
            system_executor_builder: SystemExecutor::new(),
            master_controller: None,
        }
    }

    pub fn init_resources(&mut self) {
        // This is the event/messaging
        self.world.ecs_world.add_resource(Messages::<E>::new());
        self.world.ecs_world.add_resource(InputMap::new());
    }

    pub fn register<T: Component>(&mut self)
    where <T as Component>::Storage : std::default::Default {
        self.world.ecs_world.register::<T>();
    }

    pub fn start_server(&mut self) {
        let mut server = Server::new(self.server_conf.clone());
        self.input_buffer = Some(server.get_input_buffer()); // Get a reference to the input buffer even after it gets moved to another thread
        spawn(move || server.main_loop());

        // Call MC init
        self.master_controller.start(&mut self.world, 0.0);
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
                panic!("Oh no! The input buffer mutex was poisoned!");
            }
        }

    }
    
    pub fn tick(&mut self) {
        let delta_time = 0.05; // This is a constant for now. Clock will be added soon.
        let instruction = self.master_controller.tick(&mut self.world, delta_time);
        match instruction {
            EngineInstruction::Run {
                run_dispatcher
            } => {
                if run_dispatcher {
                    let inputs = self.get_inputs();
                    self.world.ecs_world.add_resource(inputs);
                    self.world.system_executor.run(&mut self.world.ecs_world);
                    self.world.ecs_world.maintain();
                }
            }
            _ => {}
        }
        
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
    
    pub fn with_mc<M: 'static>(mut self, master_controller: M) -> Self
    where
        M: MasterController<ObserverEvent=E> {
        self.master_controller = Some(Box::new(master_controller));
        self
    }
    
    pub fn build(mut self) -> Option<Engine<'a, 'b, E>> {
        let mut engine = Engine {
            world: World {
                system_executor: self.system_executor_builder.build(),
                ecs_world: specs::prelude::World::new(),
                connections: ConnectionCollection::new(),
            },
            master_controller: self.master_controller?,
            server_conf: self.server_conf,
            input_buffer: None,
        };
        engine.init_resources();
        Some(engine)
    }
}