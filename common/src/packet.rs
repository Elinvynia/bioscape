use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct ServerPacket {
    pub command: ServerCommand,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum ServerCommand {
    Pong(u16),
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct ClientPacket {
    pub command: ClientCommand,
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub enum ClientCommand {
    Ping(u16),
}
