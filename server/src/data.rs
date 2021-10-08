use bioscape_common::{ClientPacket, ServerPacket};
use std::net::SocketAddr;

#[allow(dead_code)]
pub enum Message {
    Disconnected(SocketAddr),
    SendTo((SocketAddr, ServerPacket)),
    ReceivedFrom((SocketAddr, ClientPacket)),
}
