use super::World;

pub trait MasterController {
    type ObserverEvent;
    fn start(&mut self, _world: &mut World, _delta_time: f32) {}
    fn tick(&mut self, _world: &mut World, _delta_time: f32) -> EngineInstruction { EngineInstruction::Run {
        run_controllers: true,
        run_observer: true,
        run_systems: true
    } }
}

pub enum EngineInstruction {
    Run {
        run_observer: bool,
        run_controllers: bool,
        run_systems: bool
    },
    Pause,
    Restart
}