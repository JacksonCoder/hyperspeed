use super::World;

pub trait Controller {
    type ObserverEvent;
    fn tick(&mut self, _world: &mut World, _delta_time: f32) {}
    fn event(&mut self, _world: &mut World, _e: &Self::ObserverEvent) {}
}