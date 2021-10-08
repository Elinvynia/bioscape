#![warn(missing_debug_implementations)]

use crate::state::MainState;
use crossbeam_channel::unbounded;
use std::thread::spawn;
use tetra::ContextBuilder;

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

    // Channel to receive messages from network.
    let (r_sender, r_receiver) = unbounded();

    // Channel to send messages to network.
    let (s_sender, s_receiver) = unbounded();

    spawn(|| network::start(s_receiver, r_sender));

    ContextBuilder::new("Bioscape", 1280, 720)
        .show_mouse(true)
        .build()?
        .run(|ctx| MainState::new(ctx, s_sender, r_receiver))
}
