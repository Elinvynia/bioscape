use async_std::net::TcpListener;
use crossbeam_channel::unbounded;
use log::error;
use std::thread::spawn;

#[macro_use]
extern crate lazy_static;

mod accept;
mod data;
use data::{Client, ClientId, Server};
mod transfer;

#[async_std::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env");
    env_logger::try_init().expect("Failed to initialize env_logger");

    let listener = TcpListener::bind("").await.expect("Failed to bind TcpListener");

    // Start thread to listen for new connections.
    let (conn_sender, conn_receiver) = unbounded();
    spawn(|| accept::accept(listener, conn_sender));

    let mut server = Server::new();

    // A server tick.
    loop {
        // Check for new connections.
        if let Ok(conn) = conn_receiver.try_recv() {
            let (client_sender, client_receiver) = unbounded();
            let (server_sender, server_receiver) = unbounded();

            let id = ClientId::new();
            let client = Client {
                id,
                packet_sender: server_sender,
                packet_receiver: client_receiver,
            };

            server.clients.insert(id, client);

            spawn(|| transfer::transfer(conn, server_receiver, client_sender));
        }

        // Check for new packets from connected clients.
        let mut received = vec![];
        for (id, client) in server.clients.iter() {
            if let Ok(packet) = client.packet_receiver.try_recv() {
                received.push((id, packet))
            }
        }

        // Process the packets
        let queue = vec![];

        // Send updated data to connected clients.
        for (id, packet) in queue {
            match server.clients.get(id) {
                Some(client) => match client.packet_sender.send(packet) {
                    Ok(()) => {}
                    Err(e) => error!("Failed to send packet to client: {}", e),
                },
                None => error!("Client does not exist: {}", id),
            }
        }
    }
}
