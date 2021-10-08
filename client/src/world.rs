use crate::network::Message;
use bioscape_common::component::TextureFile;
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
    pub client_sender: Sender<Message>,
    pub server_receiver: Receiver<Message>,
}

impl GameWorld {
    pub fn new(ctx: &mut Context, client_sender: Sender<Message>, server_receiver: Receiver<Message>) -> Self {
        GameWorld {
            ecs: World::new(),
            textures: HashMap::new(),
            camera: Camera::with_window_size(ctx),
            client_sender,
            server_receiver,
        }
    }
}
