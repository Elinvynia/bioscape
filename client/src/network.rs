use bioscape_common::{ServerPacket, ServerCommand, ClientPacket};
use bincode::{serialize, deserialize};
use crossbeam_channel::{Sender, Receiver};
use std::io::prelude::*;
use std::net::TcpStream;

pub fn start(client_receiver: Receiver<ClientPacket>, server_sender: Sender<ServerPacket>) {



}

pub fn reader(mut stream: TcpStream, server_sender: Sender<ServerPacket>) {
    // Hack
    let packet = ServerPacket {
        command: ServerCommand::AddComponent
    };
    let size = serialize(&packet).unwrap().len();

    let mut buffer = vec![0u8; size];
    loop {
        stream.read_exact(&mut buffer).expect("Failed to read data.");
        let packet = deserialize(&buffer).expect("Failed to deserialize packet.");
        server_sender.send(packet).expect("Failed to send received packet.");

        buffer = vec![0u8; size]
    }
}

pub fn writer(mut stream: TcpStream, client_receiver: Receiver<ClientPacket>) {
    loop {
        if let Ok(packet) = client_receiver.recv() {
            let serialized = serialize(&packet).expect("Failed to serialize packet.");
            stream.write(&serialized).expect("Failed to send data.");
        }
    }
}
