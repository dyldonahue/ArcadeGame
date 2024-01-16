use ggez::*; //todo find exact imports needed
use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};
use crate::glam::*;

mod grid;

struct State {
    dt: std::time::Duration,
    grid_drawn: bool
}

impl State {
    fn new(ctx: &mut Context) -> State {
        let s = State {
            dt: std::time::Duration::new(0, 0),
            grid_drawn: false
        };
        s
    }

}
fn main() {

    let c = conf::Conf::new();
    let (mut ctx, event_loop) = ContextBuilder::new("Steamboat Willie x Dig Dug", "dyl")
        .default_conf(c)
        .build()
        .unwrap();
    let state = State::new(&mut ctx);
    event::run(ctx, event_loop, state);
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if !self.grid_drawn {
            grid::draw_grid(ctx, grid::NUM_HORZ, grid::NUM_VERT)?;
            self.grid_drawn = true;
        }
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