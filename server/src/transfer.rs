use crate::{Connection, ServerMessage};
use bincode::{deserialize, serialize};
use bioscape_common::{ClientCommand, ClientPacket, ServerPacket};
use crossbeam_channel::{Receiver, Sender};
use log::{error, info};
use std::io::prelude::*;

// Thread.
pub fn reader(mut conn: Connection, client_sender: Sender<ClientPacket>, message_sender: Sender<ServerMessage>) {
    // This hack is required to make the underlying TCP connection behave in a packet-like way.
    let packet = ClientPacket {
        command: ClientCommand::Move(0),
    };
    let size = serialize(&packet).unwrap().len();

    let mut buffer = vec![0u8; size];
    loop {
        match conn.stream.read_exact(&mut buffer) {
            Ok(_n) => {
                let packet = match deserialize(&buffer) {
                    Ok(p) => p,
                    Err(e) => {
                        error!("Failed to deserialize packet: {:?} ({:?})", e, &buffer);
                        message_sender.send(ServerMessage::Disconnect(conn.addr)).unwrap();
                        return;
                    }
                };

                info!("Received packet: {:?}", &packet);

                if let Err(e) = client_sender.send(packet) {
                    error!("Failed to send received packet {:?}", e);
                    message_sender.send(ServerMessage::Disconnect(conn.addr)).unwrap();
                    return;
                }
            }
            Err(e) => {
                error!("Failed to read packet from stream: {:?}", e);
                message_sender.send(ServerMessage::Disconnect(conn.addr)).unwrap();
                return;
            }
        }

        buffer = vec![0u8; size];
    }
}

// Thread.
pub fn writer(mut conn: Connection, server_receiver: Receiver<ServerPacket>, message_sender: Sender<ServerMessage>) {
    loop {
        match server_receiver.recv() {
            Ok(p) => {
                let serialized = match serialize(&p) {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Failed to serialize packet: {:?}", e);
                        message_sender.send(ServerMessage::Disconnect(conn.addr)).unwrap();
                        return;
                    }
                };

                match conn.stream.write_all(&serialized) {
                    Ok(_n) => info!("Sent packet: {:?}", &p),
                    Err(e) => {
                        error!("Failed to send packet to stream: {:?}", e);
                        message_sender.send(ServerMessage::Disconnect(conn.addr)).unwrap();
                        return;
                    }
                }
            }
            Err(e) => {
                error!("Failed to receive packet to send: {:?}", e);
                message_sender.send(ServerMessage::Disconnect(conn.addr)).unwrap();
                return;
            }
        };
    }
}
