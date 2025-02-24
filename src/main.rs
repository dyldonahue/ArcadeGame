use ggez::graphics::{Canvas, Color, Mesh, DrawParam};
use ggez::{Context, GameResult, GameError, ContextBuilder, conf, event};
use ggez::input::keyboard::{KeyCode, KeyMods, KeyInput};
use std::path::Path;

mod grid;
mod sprite;
use crate::grid::CELL_SIZE;

struct State {
    grid: Option<Mesh>,
    sprite: Option<sprite::Sprite>,
    dt: std::time::Duration,
    grid_drawn: bool
}

impl State {
    fn new(ctx: &mut Context) -> State {
        let s = State {
            sprite: None,
            dt: std::time::Duration::new(0, 0),
            grid_drawn: false,
            grid : grid::draw_grid(ctx, 800, 600).ok()

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
    
    let resources_dir = env!("CARGO_MANIFEST_DIR").to_owned() + "/resources";
    ctx.fs.mount(Path::new(&resources_dir), true);
    let state = State::new(&mut ctx);
    event::run(ctx, event_loop, state);
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {

        let mut canvas = Canvas::from_frame(ctx, Color::BLACK);

        if let Some(ref mesh) = self.grid {
            canvas.draw(mesh, DrawParam::default());
        }
           
        if self.sprite.is_none() {
            let new_sprite = sprite::Sprite::new(ctx, "sprite.webp", 0.0, 100.0, 32, 32);
            self.sprite = Some(new_sprite.unwrap());
            
        }

        if let Some(ref mut sprite) = self.sprite {
            sprite.draw_sprite(ctx, &mut canvas)?;
        }      

        canvas.finish(ctx)?;

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

            Some(KeyCode::Up) => {
                if let Some(ref mut sprite) = self.sprite {
                    sprite.move_sprite(ctx, 0.0, -CELL_SIZE);
                    grid::dig_cell(ctx, sprite.x, sprite.y, &mut self.grid.as_mut().unwrap());
                    println!("up");
                }
                
            }
            Some(KeyCode::Down) => {
                if let Some(ref mut sprite) = self.sprite {
                    sprite.move_sprite(ctx, 0.0, CELL_SIZE);
                    grid::dig_cell(ctx, sprite.x, sprite.y, &mut self.grid.as_mut().unwrap());
                    println!("down");
                }
                
            }
            Some(KeyCode::Left) => {
                if let Some(ref mut sprite) = self.sprite {
                    sprite.move_sprite(ctx, -CELL_SIZE, 0.0);
                    grid::dig_cell(ctx, sprite.x, sprite.y, &mut self.grid.as_mut().unwrap());
                }
                
            }
            Some(KeyCode::Right) => {
                if let Some(ref mut sprite) = self.sprite {
                    sprite.move_sprite(ctx, CELL_SIZE, 0.0);
                    grid::dig_cell(ctx, sprite.x, sprite.y, &mut self.grid.as_mut().unwrap());
                }
                
            }
            _ => (),
        }
        Ok(())
    }
}