#[derive(Clone, PartialEq, Eq)]
pub enum Input {
    Click { x: u32, y: u32 },
    Key(String)
}