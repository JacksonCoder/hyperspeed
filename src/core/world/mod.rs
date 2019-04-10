mod connection;
mod events;
mod input;
mod mc;
mod observer;
mod system;

pub use connection::*;
pub use events::*;
pub use input::*;
pub use mc::*;
pub use observer::*;
pub use system::*;

pub struct World<'a, 'b> {
    pub(crate) system_executor: SystemExecutor<'a, 'b>,
    pub ecs_world: specs::prelude::World,
    pub connections: ConnectionCollection,
}