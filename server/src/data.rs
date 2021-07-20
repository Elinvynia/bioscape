use async_std::net::{SocketAddr, TcpStream};
use bioscape_common::{ClientPacket, ServerPacket};
use crossbeam_channel::{Receiver, Sender};
use hecs::World;
use std::collections::HashMap;
use std::fmt;
use std::sync::Mutex;

pub struct Server {
    pub clients: HashMap<ClientId, Client>,
    pub world: World,
}

impl Server {
    pub fn new() -> Self {
        Server {
            clients: HashMap::new(),
            world: World::new(),
        }
    }
}

lazy_static! {
    static ref INCREMENT: Mutex<u64> = Mutex::new(1);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ClientId(u64);

impl ClientId {
    pub fn new() -> Self {
        let mut m = INCREMENT.lock().expect("Failed to aquire INCREMENT mutex");
        let n = *m;
        *m += 1;
        ClientId(n)
    }
}

impl fmt::Display for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct Client {
    pub id: ClientId,
    pub packet_receiver: Receiver<ClientPacket>,
    pub packet_sender: Sender<ServerPacket>,
}

pub struct Connection {
    pub stream: TcpStream,
    pub addr: SocketAddr,
}
