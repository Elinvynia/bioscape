use crossbeam_channel::unbounded;
use tokio::net::TcpListener;
use tokio::task::spawn;
use tracing::info;

mod accept;
use accept::{reader, writer};
mod data;
use data::{Client, Message, Server};

#[tokio::main]
async fn main() {
    // Environment setup.
    dotenv::dotenv().expect("Failed to initialize .env");

    // Logging setup.
    tracing_subscriber::fmt::init();

    // Create channel for sending new connections.
    let (conn_sender, conn_receiver) = unbounded();

    // Bind a TcpListener and start listening for connections.
    let listener = TcpListener::bind("127.0.0.1:5555")
        .await
        .expect("Failed to bind TcpListener");
    spawn(accept::listen(listener, conn_sender.clone()));

    // Create the server struct
    let mut server = Server::new();

    // Main server loop.
    loop {
        // Check for new connections
        if let Ok(conn) = conn_receiver.try_recv() {
            let (to_read_sender, to_read_receiver) = unbounded();
            let (from_read_sender, from_read_receiver) = unbounded();

            let (to_write_sender, to_write_receiver) = unbounded();
            let (from_write_sender, from_write_receiver) = unbounded();

            let (read, write) = conn.0.into_split();

            let client = Client {
                to_reader: to_read_sender,
                from_reader: from_read_receiver,
                to_writer: to_write_sender,
                from_writer: from_write_receiver,
            };

            server.clients.insert(conn.1, client);

            spawn(reader(read, from_read_sender, to_read_receiver));
            spawn(writer(write, from_write_sender, to_write_receiver));
        }

        // Read incoming packets
        for client in server.clients.iter() {
            if let Ok(mess) = client.1.from_reader.try_recv() {
                info!("Received message: {:?}", mess);
            }
        }
    }
}
