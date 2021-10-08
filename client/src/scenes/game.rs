use crate::network::Message;
use crate::scenes::{PauseScene, Scene, SceneSwitch, Scenes};
use crate::systems::render_system;
use crate::utils::position;
use crate::world::GameWorld;
use bioscape_common::{ClientCommand, ClientPacket};
use egui::{pos2, vec2, CtxRef, Window};
use tetra::graphics::set_transform_matrix;
use tetra::input::Key;
use tetra::window::get_size;
use tetra::{Context, Event};

#[derive(Debug)]
pub struct GameScene {
    pause: bool,
}

impl GameScene {
    pub fn new(world: &mut GameWorld, ctx: &mut Context) -> Self {
        let (width, height) = get_size(ctx);

        world.camera.position = [width as f32 / 2.0, height as f32 / 2.0].into();
        world.camera.update();

        // Start networking.
        world.start_sender.send(Message::Start).unwrap();

        GameScene { pause: false }
    }
}

impl Scene for GameScene {
    fn update(&mut self, world: &mut GameWorld, ctx: &mut Context) -> tetra::Result<SceneSwitch> {
        if self.pause {
            self.pause = false;
            let scene = Scenes::Pause(PauseScene::new(world, ctx));
            return Ok(SceneSwitch::Push(scene));
        }

        Ok(SceneSwitch::None)
    }

    fn draw(&mut self, world: &mut GameWorld, ctx: &mut Context, ectx: &mut CtxRef) -> tetra::Result {
        set_transform_matrix(ctx, world.camera.as_matrix());
        render_system(ctx, world);

        let pos = world.camera.project([100.0, 100.0].into());

        let rect = position(pos2(pos.x, pos.y), vec2(150.0, 100.0));

        Window::new("Network Test")
            .resizable(false)
            .fixed_rect(rect)
            .show(ectx, |ui| {
                let button = ui.button("Send Packet");
                if button.clicked() {
                    let command = ClientCommand::Ping(42);
                    let packet = ClientPacket { command };
                    world.send_packet(packet);
                };
            });

        Ok(())
    }

    fn event(&mut self, _world: &mut GameWorld, _ctx: &mut Context, event: Event) -> tetra::Result {
        if let Event::KeyPressed { key } = event {
            if key == Key::Escape {
                self.pause = true;
            }
        }

        Ok(())
    }
}
