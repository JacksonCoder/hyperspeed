use super::*;

pub type ZLevelID = &'static str;
pub type SpriteID = u64;

pub struct Position {
    x: f32,
    y: f32,
    z_level: ZLevelID
}

pub struct Visible {
    sprite: SpriteID
}

pub struct PositionTiled {
    x: u32,
    y: u32,
    z_level: ZLevelID
}

pub struct Camera {
    view_range: u16,
    offset: (u32, u32)
}

define_component!(Position);
define_component!(Visible);
define_component!(PositionTiled);
define_component!(Camera);