use super::World;

/// Controllers are a unique feature of Hyperspeed. Controllers can be seen
/// as 'meta-systems'. They are for scenarios like global subsystems that need to
/// modify the entire entity system and also respond to events that are emitted
/// by an Observer.
pub trait Controller {
    type ObserverEvent;
    fn tick(&mut self, _world: &mut World, _delta_time: f32) {}
    fn event(&mut self, _world: &mut World, _e: &Self::ObserverEvent) {}
}