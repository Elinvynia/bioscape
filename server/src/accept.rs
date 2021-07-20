use crate::data::Connection;
use async_std::net::TcpListener;
use crossbeam_channel::Sender;
use log::error;

// Thread.
pub async fn accept(listener: TcpListener, conn_sender: Sender<Connection>) {
    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                let conn = Connection { stream, addr };
                if let Err(e) = conn_sender.send(conn) {
                    error!("Failed to send incoming connection: {}", e);
                }
            }
            Err(e) => error!("Failed to accept incoming connection: {}", e),
        }
    }
}
