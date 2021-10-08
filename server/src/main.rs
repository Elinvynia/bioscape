use crossbeam_channel::unbounded;
use hecs::World;
use tokio::net::TcpListener;
use tokio::task::spawn;

mod accept;
mod data;
use data::Message;

#[tokio::main]
async fn main() {
    // Environment setup.
    dotenv::dotenv().expect("Failed to initialize .env");

    // Logging setup.
    tracing_subscriber::fmt::init();

    // Channel to receive messages from tasks.
    let (r_sender, r_receiver) = unbounded::<Message>();

    // Channel to send messages to tasks.
    let (s_sender, s_receiver) = unbounded::<Message>();

    // Bind a TcpListener and start listening for connections.
    let listener = TcpListener::bind("127.0.0.1:5555")
        .await
        .expect("Failed to bind TcpListener");
    spawn(accept::listen(listener, r_sender.clone(), s_receiver.clone()));

    let _world = World::new();

    // Main server loop.
    loop {
        // Check for new messages.
        while let Ok(_message) = r_receiver.try_recv() {}

        // Process packets.
        let messages = vec![];

        // Send messages.
        for message in messages {
            s_sender.send(message).expect("Failed to send message.");
        }
    }
}
