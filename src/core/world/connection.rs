pub struct ConnectionCollection {
    pub connections: Vec<Connection>
}

// Sharing connections can be hard, because both the ECS system and the server
// need to read/write to the client views. In this case, the world's connection collection
// is passed immutably to the server, which sends data to each respective channel.
pub struct Connection {
    pub key: String
}

pub struct ClientView {
    pub data: String
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