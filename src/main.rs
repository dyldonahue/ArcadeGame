use ggez::*; //todo find exact imports needed
use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};

mod grid;

struct State {
    dt: std::time::Duration,
    //bg: ggez::graphics::Mesh
}

fn main() {
    let state = State {
        dt: std::time::Duration::new(0, 0),
       // bg: grid::create_grid() // TODO need to gert graphics context
    };

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("arcade_game", "dyl")
        .default_conf(c)
        .build()
        .unwrap();

        event::run(ctx, event_loop, state);
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
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