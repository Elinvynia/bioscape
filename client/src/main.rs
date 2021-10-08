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

    // Channel to receive messages from reader.
    let (r_r_sender, r_r_receiver) = unbounded();
    // Channel to send messages to reader.
    let (s_r_sender, s_r_receiver) = unbounded();

    // Channel to receive messages from writer.
    let (r_w_sender, r_w_receiver) = unbounded();
    // Channel to send messages to writer.
    let (s_w_sender, s_w_receiver) = unbounded();

    // Channel used to start networking.
    let (start_sender, start_receiver) = unbounded();

    spawn(|| network::start(start_receiver, s_r_receiver, r_r_sender, s_w_receiver, r_w_sender));

    ContextBuilder::new("Bioscape", 1280, 720)
        .show_mouse(true)
        .build()?
        .run(|ctx| MainState::new(ctx, start_sender, s_r_sender, r_r_receiver, s_w_sender, r_w_receiver))
}
