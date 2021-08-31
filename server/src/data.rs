use bioscape_common::{ClientPacket, ServerPacket};
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;
use std::net::{SocketAddr, TcpStream};

#[derive(Debug)]
pub struct Server {
    pub clients: HashMap<SocketAddr, Client>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    pub fn add_client(
        &mut self,
        addr: SocketAddr,
        client_receiver: Receiver<ClientPacket>,
        server_sender: Sender<ServerPacket>,
    ) {
        let client = Client::new(client_receiver, server_sender);
        self.clients.insert(addr, client);
    }

    pub fn remove_client(&mut self, addr: SocketAddr) {
        self.clients.remove(&addr);
    }
}

#[derive(Debug)]
pub enum ServerMessage {
    Disconnect(SocketAddr),
}

#[derive(Debug)]
pub struct Client {
    pub client_receiver: Receiver<ClientPacket>,
    pub server_sender: Sender<ServerPacket>,
}

impl Client {
    pub fn new(client_receiver: Receiver<ClientPacket>, server_sender: Sender<ServerPacket>) -> Self {
        Self {
            client_receiver,
            server_sender,
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    pub addr: SocketAddr,
    pub stream: TcpStream,
}

impl Connection {
    pub fn new(addr: SocketAddr, stream: TcpStream) -> Self {
        Self { addr, stream }
    }

    pub fn try_clone(&self) -> std::io::Result<Self> {
        let stream = self.stream.try_clone()?;
        let conn = Connection::new(self.addr, stream);
        Ok(conn)
    }
}
