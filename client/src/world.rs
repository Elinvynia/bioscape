use crate::network::Message;
use bioscape_common::component::TextureFile;
use bioscape_common::ClientPacket;
use crossbeam_channel::{Receiver, Sender};
use hecs::World;
use std::collections::HashMap;
use tetra::graphics::{Camera, Texture};
use tetra::Context;

// Mutable global state, used in every scene.
pub struct GameWorld {
    pub ecs: World,
    pub textures: HashMap<TextureFile, Texture>,
    pub camera: Camera,
    pub start_sender: Sender<Message>,
    pub reader_sender: Sender<Message>,
    pub reader_receiver: Receiver<Message>,
    pub writer_sender: Sender<Message>,
    pub writer_receiver: Receiver<Message>,
}

impl GameWorld {
    pub fn new(
        ctx: &mut Context,
        start_sender: Sender<Message>,
        reader_sender: Sender<Message>,
        reader_receiver: Receiver<Message>,
        writer_sender: Sender<Message>,
        writer_receiver: Receiver<Message>,
    ) -> Self {
        GameWorld {
            ecs: World::new(),
            textures: HashMap::new(),
            camera: Camera::with_window_size(ctx),
            start_sender,
            reader_sender,
            reader_receiver,
            writer_sender,
            writer_receiver,
        }
    }

    pub fn send_packet(&self, packet: ClientPacket) {
        self.writer_sender.send(Message::Send(packet)).unwrap();
    }
}
