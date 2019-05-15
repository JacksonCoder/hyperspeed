use super::system::*;
use super::connection::*;
use crate::{ViewMap, InputMap};
use specs::{World as ECSWorld, Component};
use std::marker::PhantomData;
use crate::components::*;
use crate::utils::*;


pub struct World<'a, 'b, E: Sync + Send + Clone + 'static> {
    system_executor: SystemExecutor<'a, 'b>,
    ecs_world: ECSWorld,
    connections: ConnectionCollection,
    view_map: ViewMap,
    input_map: InputMap,
    _p_marker: PhantomData<E>
}

impl<'a, 'b, E: Sync + Send + Clone + 'static> World<'a, 'b, E> {
    pub fn new(system_executor: SystemExecutorBuilder<'a, 'b>) -> Self {
        World {
            system_executor: system_executor.build(),
            ecs_world: ECSWorld::new(),
            connections: ConnectionCollection::new(),
            view_map: ViewMap::new(),
            input_map: InputMap::new(),
            _p_marker: PhantomData
        }
    }

    pub fn get_world(&self) -> &ECSWorld {
        &self.ecs_world
    }

    pub fn get_world_mut(&mut self) -> &mut ECSWorld {
        &mut self.ecs_world
    }

    pub fn get_connections(&self) -> &ConnectionCollection {
        &self.connections
    }

    pub fn init_resources(&mut self) {
        // Register resources needed by the engine
        self.ecs_world.add_resource(Messages::<E>::new());
        self.ecs_world.add_resource(InputMap::new());
        self.ecs_world.add_resource(ViewMap::new());
        self.ecs_world.add_resource(ConnectionCollection::new());

        // Register default components

        self.ecs_world.register::<Position>();
        self.ecs_world.register::<Visible>();
        self.ecs_world.register::<Camera>();
    }

    pub fn add_connection(&mut self, conn: Connection) {
        self.connections.push(conn)
    }

    pub fn update_connections_in_world(&mut self) {
        let mut conn_ref = self.ecs_world.write_resource::<ConnectionCollection>();
        ::std::mem::swap(&mut *conn_ref, &mut self.connections);
        drop(conn_ref);
    }

    pub fn remove_connection(&mut self, key: &String) {
        self.connections.remove(key);
    }

    pub fn set_input(&mut self, input: InputMap) {
        self.ecs_world.add_resource(input);
    }

    pub fn tick(&mut self) {
        self.system_executor.run(&mut self.ecs_world);
    }

    pub fn maintain(&mut self) {
        self.ecs_world.maintain();
    }

    pub fn pop_views(&mut self) -> ViewMap {
        // Get views
        let mut view_ref = self.ecs_world.write_resource::<ViewMap>();
        let mut views = ViewMap::new();
        // Swap views
        ::std::mem::swap(&mut *view_ref, &mut views);
        drop(view_ref);
        views
    }

    pub fn register<C>(&mut self)
    where C: Component, <C as Component>::Storage: std::default::Default {
        self.ecs_world.register::<C>();
    }
}