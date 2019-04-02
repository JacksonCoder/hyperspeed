use super::World;
use super::Events;
use super::Input;

use std::collections::HashMap;

pub trait Observer {
    type ObserverEvent;
    // TODO: Make HashMap<String, Vec<Input>> encapsulated inside of an InputCollection or something.
    fn observe_world(&mut self, _world: &mut World, _inputs: HashMap<String, Vec<Input>>) -> Events<Self::ObserverEvent> {
        Events::new()
    }
}