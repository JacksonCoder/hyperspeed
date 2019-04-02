#[derive(Default)]
pub struct Events<E> {
    events: Vec<E>
}

impl<E> Events<E> {
    pub fn new() -> Events<E> {
        Events {
            events: vec!()
        }
    }
}