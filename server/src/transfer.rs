use crate::data::Connection;
use async_std::net::TcpStream;
use async_std::prelude::*;
use bioscape_common::{ClientPacket, ServerPacket};
use crossbeam_channel::{Receiver, Sender};
use futures::join;
use log::error;

// Thread.
pub async fn transfer(
    connection: Connection,
    server_receiver: Receiver<ServerPacket>,
    client_sender: Sender<ClientPacket>,
) {
    let stream_read = &connection.stream;
    let stream_write = &connection.stream;

    let reader = reader(stream_read, client_sender);
    let writer = writer(stream_write, server_receiver);

    join!(reader, writer);
}

// Blocking.
pub async fn reader(mut stream: &TcpStream, client_sender: Sender<ClientPacket>) {
    let mut buffer = vec![0u8; 1024];
    loop {
        match stream.read(&mut buffer).await {
            Ok(_) => {
                let packet = match bincode::deserialize(&buffer) {
                    Ok(p) => p,
                    Err(e) => {
                        error!("Failed to serialize data to packet: {}", e);
                        continue;
                    }
                };

                match client_sender.send(packet) {
                    Ok(_) => {}
                    Err(e) => {
                        error!("Failed to send received packet: {}", e);
                        continue;
                    }
                }

                buffer.clear();
            }
            Err(e) => error!("Failed to read data from client: {}", e),
        }
    }
}

// Blocking.
pub async fn writer(mut stream: &TcpStream, server_receiver: Receiver<ServerPacket>) {
    loop {
        let packet = match server_receiver.recv() {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to receive packet: {}", e);
                continue;
            }
        };

        let serialized = bincode::serialize(&packet).expect("Failed to serialize valid packet.");

        if let Err(e) = stream.write(&serialized).await {
            error!("Failed to send packet to client: {}", e);
        };
    }
}
