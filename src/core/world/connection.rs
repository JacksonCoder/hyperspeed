pub struct ConnectionCollection {
    connections: Vec<Connection>
}

pub struct Connection {
    pub key: String,
    // This ClientView is sent to the server at the end of each tick.
    pub view: ClientView
}

pub struct ClientView {

}

impl ConnectionCollection {
    pub fn new() -> Self {
        ConnectionCollection {
            connections: vec![]
        }
    }

    pub fn size(&self) -> usize {
        self.connections.len()
    }
}