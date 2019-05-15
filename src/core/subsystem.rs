use super::World;
use std::ops::{Mul, MulAssign};

/// Controls and provides an interface for high-level manipulation of the engine and game state.
/// This can be used to implement your own custom framework embedded in the engine and also regulate rebooting/restarting
/// the game.
pub trait Subsystem {

    /// The type used for messaging within the world.
    type MessageType: Send + Sync + Clone + 'static;

    /// The handler function when the game engine starts processing.
    fn start(&mut self, _world: &mut World<Self::MessageType>) {}
    /// The handler function that runs right before the world tick. If you return `Pause` from
    /// this function, this will not run the world tick.
    /// You can use `delta_time` to find the amount of time since the last tick, which is
    /// useful for FPS regulation.
    ///
    /// If you return `Restart` from this function, the engine will panic, as this is not an implemented feature.
    fn tick(&mut self, _world: &mut World<Self::MessageType>, _delta_time: f64) -> EngineInstruction {
        EngineInstruction::Run (
            RunData {
                run_dispatcher: true
            },
            1
        )
    }
    /// This handler function executes right before an engine restart.
    fn restart(&mut self, _world: &mut World<Self::MessageType>) {}
}

/// The instructions
pub enum EngineInstruction {
    Run(RunData, u32),
    Pause(u32),
    Restart(u32)
}

pub struct RunData {
    pub run_dispatcher: bool
}

use self::EngineInstruction::*;

impl Mul for EngineInstruction {
    type Output = EngineInstruction;

    fn mul(self, rhs: EngineInstruction) -> Self::Output {
        let priority_1 = match self {
            Run(_, p1) => p1,
            Pause(p1) => p1,
            Restart(p1) => p1
        };
        let priority_2 = match rhs {
            Run(_, p2) => p2,
            Pause(  p2) => p2,
            Restart(p2) => p2
        };
        if priority_1 >= priority_2 {
            self
        } else {
            rhs
        }
    }
}

impl MulAssign for EngineInstruction {
    fn mul_assign(&mut self, rhs: EngineInstruction) {
        let mut tmp = Pause(0);
        std::mem::swap(&mut tmp, self);
        let tmp = tmp * rhs;
        *self = tmp;
    }
}