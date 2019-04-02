pub(crate) struct Server {
    
}

pub(crate) struct ServerConfig {
    pub port: u16,
    pub server_name: String
}

impl ServerConfig {
    pub fn new() -> Self {
        ServerConfig {
            port: 0,
            server_name: "default_name".to_string()
        }
    }
}

impl Server {
    pub(crate) fn new(s: ServerConfig) -> Server {
        Server {}
    }
}