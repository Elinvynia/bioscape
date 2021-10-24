use bioscape_common::{ClientPacket, ServerPacket};
use crossbeam_channel::{Receiver, Sender};
use log::{error, info};
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{Shutdown, TcpStream};
use std::thread::spawn;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Message {
    Disconnect,
    Received(ServerPacket),
    Start,
    Send(ClientPacket),
}

pub fn start(
    start_receiver: Receiver<Message>,
    reader_receiver: Receiver<Message>,
    reader_sender: Sender<Message>,
    writer_receiver: Receiver<Message>,
    writer_sender: Sender<Message>,
) {
    loop {
        if let Ok(Message::Start) = start_receiver.try_recv() {
            let stream_one = TcpStream::connect("127.0.0.1:5555").unwrap();
            let _ = stream_one.set_nodelay(true);

            let stream_two = stream_one.try_clone().unwrap();

            spawn(move || reader(stream_one, reader_receiver, reader_sender));
            spawn(move || writer(stream_two, writer_receiver, writer_sender));
            return;
        }
    }
}

pub fn reader(stream: TcpStream, message_receiver: Receiver<Message>, message_sender: Sender<Message>) {
    let mut stream = BufReader::new(stream);
    let mut buffer = String::with_capacity(100);

    loop {
        if let Ok(message) = message_receiver.try_recv() {
            info!("Reader received message: {:?}", &message);
            if let Message::Disconnect = message {
                let _ = stream.get_mut().shutdown(Shutdown::Read);
                return;
            }
        }

        if let Err(e) = stream.read_line(&mut buffer) {
            let _ = stream.get_mut().shutdown(Shutdown::Read);
            error!("Failed to read data from server: {:?}", e);
            return;
        }

        let packet: ServerPacket = match serde_json::from_str(&buffer) {
            Ok(p) => p,
            Err(e) => {
                let _ = stream.get_mut().shutdown(Shutdown::Read);
                error!("Failed to deserialize server packet: {:?}", e);
                return;
            }
        };

        if let Err(e) = message_sender.send(Message::Received(packet)) {
            let _ = stream.get_mut().shutdown(Shutdown::Read);
            error!("Failed to send packet for processing: {:?}", e);
            return;
        }
    }
}

pub fn writer(mut stream: TcpStream, message_receiver: Receiver<Message>, _message_sender: Sender<Message>) {
    loop {
        if let Ok(message) = message_receiver.try_recv() {
            info!("Writer received message: {:?}", &message);
            match message {
                Message::Disconnect => {
                    let _ = stream.shutdown(Shutdown::Write);
                    return;
                }
                Message::Send(packet) => {
                    let mut serialized = match serde_json::to_vec(&packet) {
                        Ok(s) => s,
                        Err(e) => {
                            let _ = stream.shutdown(Shutdown::Read);
                            error!("Failed to serialize client packet: {:?}", e);
                            return;
                        }
                    };

                    serialized.push(b'\n');

                    if let Err(e) = stream.write_all(&serialized) {
                        let _ = stream.shutdown(Shutdown::Read);
                        error!("Failed to send data to server: {:?}", e);
                        return;
                    }
                }
                _ => {}
            }
        }
    }
}
