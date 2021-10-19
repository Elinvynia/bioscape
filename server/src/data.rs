use bioscape_common::{ClientPacket, ServerPacket};
use crossbeam_channel::{Receiver, Sender};
use std::collections::HashMap;
use std::net::SocketAddr;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Message {
    Disconnected,
    Send(ServerPacket),
    Received(ClientPacket),
}

#[derive(Debug)]
pub struct Server {
    pub clients: HashMap<SocketAddr, Client>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            clients: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Client {
    pub from_reader: Receiver<Message>,
    pub to_reader: Sender<Message>,
    pub from_writer: Receiver<Message>,
    pub to_writer: Sender<Message>,
}
