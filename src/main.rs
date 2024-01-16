use ggez::*; //todo find exact imports needed
use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};
use crate::glam::*;

mod grid;

struct State {
    dt: std::time::Duration,
    bg_mesh: ggez::graphics::Mesh,
    bg_batch: ggez::graphics::InstanceArray
}

fn main() {

    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("Steamboat Willie x Dig Dug", "dyl")
        .default_conf(c)
        .build()
        .unwrap();
    let state = State {
        dt: std::time::Duration::new(0, 0),
        bg_mesh: grid::create_grid(&mut ctx, 10, 10, ggez::graphics::Color::new(1.0, 1.0, 1.0, 1.0)),
        bg_batch: grid::batch_grid(&mut ctx)
           
        };
        event::run(ctx, event_loop, state);
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::WHITE);
        canvas.draw_instanced_mesh(self.bg_mesh.clone(), &self.bg_batch, graphics::DrawParam::new().dest(Vec2::new(5.0, 8.0)));
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        match input.keycode {
        // Quit if Shift+Ctrl+Q is pressed.
            Some(KeyCode::Q) => {
                if input.mods.contains(KeyMods::SHIFT) && input.mods.contains(KeyMods::CTRL) {
                    println!("Terminating!");
                    ctx.request_quit();
                } else if input.mods.contains(KeyMods::SHIFT) || input.mods.contains(KeyMods::CTRL) {
                    println!("You need to hold both Shift and Control to quit.");
                } else {
                    println!("Now you're not even trying!");
                }
            }
            _ => (),
        }
        Ok(())
    }
}