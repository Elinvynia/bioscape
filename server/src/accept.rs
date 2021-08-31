use crate::Connection;
use crossbeam_channel::Sender;
use log::{error, info};
use std::net::TcpListener;

// Thread.
pub fn accept(listener: TcpListener, conn_sender: Sender<Connection>) {
    info!("Listening for connections");
    loop {
        match listener.accept() {
            Ok((stream, addr)) => {
                let connection = Connection::new(addr, stream);
                if let Err(e) = conn_sender.send(connection) {
                    error!("Failed to send incoming connection: {}", e);
                }
                info!("Connection received: {}", addr);
            }
            Err(e) => error!("Failed to accept incoming connection: {}", e),
        }
    }
}
