use super::world::*;
use super::Server;
use super::ServerConfig;

use std::collections::HashMap;

pub struct Engine<'a, 'b, E: Sync + Send + 'static> {
    pub world: World<'a, 'b>,
    server: Server,
    observer: Box<Observer<ObserverEvent=E>>,
    master_controller: Box<MasterController<ObserverEvent=E>>
}

pub struct EngineBuilder<'a, 'b, E: Sync + Send + 'static> {
    server_conf: ServerConfig,
    system_executor_builder: SystemExecutorBuilder<'a, 'b>,
    observer: Option<Box<Observer<ObserverEvent=E>>>,
    master_controller: Option<Box<MasterController<ObserverEvent=E>>>
}

impl<'a, 'b, E: Sync + Send + 'static> Engine<'a, 'b, E> {
    pub fn new() -> EngineBuilder<'a, 'b, E> {
        EngineBuilder {
            server_conf: ServerConfig::new(),
            system_executor_builder: SystemExecutor::new(),
            observer: None,
            master_controller: None
        }
    }
    
    pub fn tick(&mut self) {
        let instruction = self.master_controller.tick(&mut self.world, 0.05);
        match instruction {
            EngineInstruction::Run {
                run_controllers,
                run_observer,
                run_systems
            } => {
                if run_controllers {
                    // TODO: impl
                }
                if run_observer {
                    let observation = self.observer.observe_world(&mut self.world, HashMap::new());
                    self.world.ecs_world.add_resource(observation);
                    if run_controllers {
                        // TODO: impl
                    }
                }
                if run_systems {
                    self.world.system_executor.run(&mut self.world.ecs_world);
                    self.world.ecs_world.maintain();
                }
            }
            _ => {}
        }
        
    }
}

impl<'a, 'b, E: Sync + Send + 'static> EngineBuilder<'a, 'b, E> {
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
    
    pub fn with_observer<O: 'static>(mut self, observer: O) -> Self 
    where
        O: Observer<ObserverEvent=E> {
        self.observer = Some(Box::new(observer));
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
                connections: ConnectionCollection {},
            },
            observer: self.observer?,
            master_controller: self.master_controller?,
            server: Server::new(self.server_conf)
        };
        engine.world.ecs_world.add_resource(Events::<E>::new());
        Some(engine)
    }
}