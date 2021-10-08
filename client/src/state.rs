use crate::egui::{handle_event, render_ui};
use crate::network::Message;
use crate::scenes::{MenuScene, SceneStack, Scenes};
use crate::world::GameWorld;
use crossbeam_channel::{Receiver, Sender};
use egui::{pos2, CtxRef, RawInput};
use tetra::graphics::{clear, Color};
use tetra::time::get_delta_time;
use tetra::{Context, Event, State};

// Semi-mutable global state, handles switching scenes.
pub struct MainState {
    scenes: SceneStack,
    egui: CtxRef,
    input: RawInput,
}

impl MainState {
    pub fn new(
        ctx: &mut Context,
        start_sender: Sender<Message>,
        reader_sender: Sender<Message>,
        reader_receiver: Receiver<Message>,
        writer_sender: Sender<Message>,
        writer_receiver: Receiver<Message>,
    ) -> tetra::Result<Self> {
        let mut world = GameWorld::new(
            ctx,
            start_sender,
            reader_sender,
            reader_receiver,
            writer_sender,
            writer_receiver,
        );
        let scene = MenuScene::new(&mut world, ctx);
        let scenes = SceneStack::new(world, Scenes::Menu(scene));

        let egui = CtxRef::default();
        let input = RawInput::default();

        Ok(MainState { scenes, egui, input })
    }
}

// Wrapper for egui, otherwise it just simply passes calls into the SceneStack.
impl State for MainState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        clear(ctx, Color::rgb(0.8, 0.8, 0.95));

        let new = match &mut self.input.time {
            Some(prev) => Some(*prev + get_delta_time(ctx).as_secs_f64()),
            None => Some(get_delta_time(ctx).as_secs_f64()),
        };

        self.input.time = new;
        self.egui.begin_frame(self.input.take());

        // Set window borders
        let mut visuals = self.egui.style().visuals.clone();
        visuals.window_shadow.extrusion = 4.0;
        self.egui.set_visuals(visuals);

        self.input.time = new;
        self.scenes.draw(ctx, &mut self.egui)?;
        let (_output, shapes) = self.egui.end_frame();
        render_ui(ctx, &mut self.egui, shapes);

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        self.scenes.update(ctx)?;
        Ok(())
    }

    fn event(&mut self, ctx: &mut Context, event: Event) -> tetra::Result {
        let pos = match self.scenes.scenes.iter().find(|x| matches!(x, Scenes::Game(_))) {
            Some(_) => {
                let pos = self.scenes.world.camera.mouse_position(ctx);
                Some(pos2(pos.x, pos.y))
            }
            _ => None,
        };

        handle_event(ctx, &mut self.input, &event, pos);
        self.scenes.event(ctx, event)?;
        Ok(())
    }
}
