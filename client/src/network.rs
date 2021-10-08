use bioscape_common::{ClientPacket, ServerPacket};
use crossbeam_channel::{Receiver, Sender};
use log::error;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{Shutdown, TcpStream};
use std::thread::spawn;

#[allow(dead_code)]
pub enum Message {
    Disconnect,
    Received(ServerPacket),
    Start,
    Send(ClientPacket),
}

pub fn start(message_receiver: Receiver<Message>, message_sender: Sender<Message>) {
    loop {
        if let Ok(message) = message_receiver.try_recv() {
            match message {
                Message::Start => {
                    let stream_one = TcpStream::connect("127.0.0.1:5555").unwrap();

                    let stream_two = stream_one.try_clone().unwrap();
                    let message_receiver_two = message_receiver.clone();
                    let message_sender_two = message_sender.clone();

                    spawn(move || reader(stream_one, message_receiver, message_sender));
                    spawn(move || writer(stream_two, message_receiver_two, message_sender_two));
                    return;
                }
                _ => {}
            }
        }
    }
}

pub fn reader(stream: TcpStream, message_receiver: Receiver<Message>, message_sender: Sender<Message>) {
    let mut stream = BufReader::new(stream);
    let mut buffer = String::with_capacity(100);

    loop {
        if let Ok(message) = message_receiver.try_recv() {
            match message {
                Message::Disconnect => {
                    let _ = stream.get_mut().shutdown(Shutdown::Read);
                    return;
                }
                _ => {}
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
}

pub fn writer(mut stream: TcpStream, message_receiver: Receiver<Message>, _message_sender: Sender<Message>) {
    loop {
        if let Ok(message) = message_receiver.try_recv() {
            match message {
                Message::Disconnect => {
                    let _ = stream.shutdown(Shutdown::Write);
                    return;
                }
                Message::Send(packet) => {
                    let serialized = match serde_json::to_vec(&packet) {
                        Ok(s) => s,
                        Err(e) => {
                            let _ = stream.shutdown(Shutdown::Read);
                            error!("Failed to serialize client packet: {:?}", e);
                            return;
                        }
                    };

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
