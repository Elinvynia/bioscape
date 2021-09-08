use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerPacket {
    pub command: ServerCommand
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ServerCommand {
    AddComponent,
    RemoveComponent
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ClientPacket {
    pub command: ClientCommand
}

#[derive(Deserialize, Serialize, Debug)]
pub enum ClientCommand {
    Move(u16),
}
