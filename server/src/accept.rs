use crate::Message;
use bioscape_common::ClientPacket;
use crossbeam_channel::{Receiver, Sender};
use std::net::SocketAddr;
use tokio::io::*;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

pub async fn listen(listener: TcpListener, sender: Sender<(TcpStream, SocketAddr)>) {
    loop {
        match listener.accept().await {
            Ok((conn, addr)) => {
                info!("Received new connection from: {}", addr);
                let _ = conn.set_nodelay(true);
                let _ = sender.send((conn, addr));
            }
            Err(e) => error!("Failed to accept new connection: {:?}", e),
        }
    }
}

pub async fn reader(stream: OwnedReadHalf, sender: Sender<Message>, receiver: Receiver<Message>) {
    let mut stream = BufReader::new(stream);
    let mut buffer = String::with_capacity(1024);

    loop {
        if let Ok(message) = receiver.try_recv() {
            use Message::*;
            match message {
                Disconnected => return,
                _ => {}
            }
        }

        if let Err(e) = stream.read_line(&mut buffer).await {
            error!("Failed to read data from client: {:?}", e);
            let _ = sender.send(Message::Disconnected);
            return;
        }

        let packet: ClientPacket = match serde_json::from_str(&buffer) {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to read data from client: {:?}", e);
                let _ = sender.send(Message::Disconnected);
                return;
            }
        };

        let message = Message::Received(packet);
        let _ = sender.send(message);

        buffer.clear();
    }
}

pub async fn writer(mut stream: OwnedWriteHalf, sender: Sender<Message>, receiver: Receiver<Message>) {
    loop {
        if let Ok(message) = receiver.try_recv() {
            use Message::*;
            match message {
                Send(packet) => {
                    let serialized = serde_json::to_vec(&packet).unwrap();
                    if let Err(e) = stream.write_all(&serialized).await {
                        error!("Failed to send data to client: {:?}", e);
                        let _ = sender.send(Message::Disconnected);
                        return;
                    }
                }
                Disconnected => return,
                _ => {}
            }
        }
    }
}
