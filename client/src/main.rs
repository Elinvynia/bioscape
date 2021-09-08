#![warn(missing_debug_implementations)]

use crate::state::MainState;
use crossbeam_channel::unbounded;
use tetra::ContextBuilder;
use std::thread::spawn;

mod egui;
mod network;
mod scenes;
mod state;
mod systems;
mod utils;
mod world;

fn main() -> tetra::Result {
    dotenv::dotenv().expect("Failed to setup dotenv.");
    env_logger::init();

    let (client_sender, client_receiver) = unbounded();
    let (server_sender, server_receiver) = unbounded();

    spawn(|| network::start(client_receiver, server_sender));

    ContextBuilder::new("Bioscape", 1280, 720)
        .show_mouse(true)
        .build()?
        .run(|ctx| MainState::new(ctx, client_sender, server_receiver))
}
