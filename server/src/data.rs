use bioscape_common::{ClientPacket, ServerPacket};
use std::net::SocketAddr;

pub enum Message {
    Disconnected(SocketAddr),
    SendTo((SocketAddr, ServerPacket)),
    ReceivedFrom((SocketAddr, ClientPacket)),
}
