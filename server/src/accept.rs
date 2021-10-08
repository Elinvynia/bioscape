use crate::Message;
use bioscape_common::ClientPacket;
use crossbeam_channel::{Receiver, Sender};
use std::net::SocketAddr;
use tokio::io::*;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::TcpListener;
use tokio::task::spawn;
use tracing::{error, info};

pub async fn listen(listener: TcpListener, sender: Sender<Message>, receiver: Receiver<Message>) {
    loop {
        match listener.accept().await {
            Ok((conn, addr)) => {
                info!("Received new connection from: {}", addr);
                let (read, write) = conn.into_split();
                spawn(reader(read, addr, sender.clone(), receiver.clone()));
                spawn(writer(write, addr, sender.clone(), receiver.clone()));
            }
            Err(e) => error!("Failed to accept new connection: {:?}", e),
        }
    }
}

pub async fn reader(stream: OwnedReadHalf, addr: SocketAddr, sender: Sender<Message>, receiver: Receiver<Message>) {
    let mut stream = BufReader::new(stream);

    let mut buffer = String::with_capacity(1024);
    loop {
        if let Ok(message) = receiver.try_recv() {
            use Message::*;
            match message {
                Disconnected(ip) if ip == addr => return,
                _ => {}
            }
        }

        if let Err(e) = stream.read_line(&mut buffer).await {
            error!("Failed to read data from client: {:?}", e);
            let _ = sender.send(Message::Disconnected(addr));
            return;
        }

        let packet: ClientPacket = match serde_json::from_str(&buffer) {
            Ok(p) => p,
            Err(e) => {
                error!("Failed to read data from client: {:?}", e);
                let _ = sender.send(Message::Disconnected(addr));
                return;
            }
        };

        let message = Message::ReceivedFrom((addr, packet));
        let _ = sender.send(message);

        buffer.clear();
    }
}

pub async fn writer(
    mut stream: OwnedWriteHalf,
    addr: SocketAddr,
    sender: Sender<Message>,
    receiver: Receiver<Message>,
) {
    loop {
        if let Ok(message) = receiver.try_recv() {
            use Message::*;
            match message {
                SendTo((dest, packet)) if addr == dest => {
                    let serialized = serde_json::to_vec(&packet).unwrap();
                    if let Err(e) = stream.write_all(&serialized).await {
                        error!("Failed to send data to client: {:?}", e);
                        let _ = sender.send(Message::Disconnected(addr));
                        return;
                    }
                }
                Disconnected(ip) if ip == addr => return,
                _ => {}
            }
        }
    }
}
