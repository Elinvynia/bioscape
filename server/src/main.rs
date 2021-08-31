use crossbeam_channel::unbounded;
use std::net::TcpListener;
use std::thread::spawn;

mod accept;
mod data;
use data::{Connection, Server, ServerMessage};
mod transfer;
use transfer::{reader, writer};

fn main() {
    dotenv::dotenv().expect("Failed to load .env");
    env_logger::try_init().expect("Failed to initialize env_logger");

    let listener = TcpListener::bind("127.0.0.1:5000").expect("Failed to bind TcpListener");

    // Start thread to listen for new connections.
    let (conn_sender, conn_receiver) = unbounded();
    spawn(|| accept::accept(listener, conn_sender));

    let (message_sender, message_receiver) = unbounded();
    let mut server = Server::new();

    // A server tick.
    loop {
        // Check for new connections.
        if let Ok(conn) = conn_receiver.try_recv() {
            let (client_sender, client_receiver) = unbounded();
            let (server_sender, server_receiver) = unbounded();

            server.add_client(conn.addr, client_receiver, server_sender);

            let conn_clone = conn.try_clone().expect("Stream clone failed");
            let message_clone = message_sender.clone();
            spawn(|| reader(conn, client_sender, message_clone));
            let message_clone = message_sender.clone();
            spawn(|| writer(conn_clone, server_receiver, message_clone));
        }

        // Check for new server messages.
        if let Ok(mess) = message_receiver.try_recv() {
            use ServerMessage::*;
            match mess {
                Disconnect(addr) => server.remove_client(addr),
            }
        }

        // Receive packets

        // Process data

        // Send packets
    }
}
