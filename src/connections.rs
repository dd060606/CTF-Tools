use std::collections::HashMap;
use std::net::TcpStream;

pub struct Connections {
    connections: HashMap<u16, TcpStream>,
}

impl Connections {
    pub fn new() -> Connections {
        let connections = HashMap::new();
        Connections { connections }
    }
    pub fn add_connection(&mut self, id: u16, stream: TcpStream) {
        self.connections.insert(id, stream);
    }

    pub fn get_connection(&mut self, id: u16) -> Option<&mut TcpStream> {
        self.connections.get_mut(&id)
    }

    pub fn remove_connection(&mut self, id: u16) {
        self.connections.remove(&id);
    }
}
