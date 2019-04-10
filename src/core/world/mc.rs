use super::World;

pub trait MasterController {
    type ObserverEvent;
    fn start(&mut self, _world: &mut World, _delta_time: f32) {}
    fn tick(&mut self, _world: &mut World, _delta_time: f32) -> EngineInstruction { EngineInstruction::Run {
        run_dispatcher: true
    } }
}

pub enum EngineInstruction {
    Run {
        run_dispatcher: bool
    },
    Pause,
    Restart
}