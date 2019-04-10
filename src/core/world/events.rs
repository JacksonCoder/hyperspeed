#[derive(Default, Clone)]
pub struct Events<E> {
    pub events: Vec<E>
}

impl<E> Events<E> {
    pub fn new() -> Events<E> {
        Events {
            events: vec!()
        }
    }
}