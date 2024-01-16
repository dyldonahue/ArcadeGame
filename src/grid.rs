use ggez::graphics::*;
use ggez::Context;
use ggez::GameResult;
use ggez::mint;
use crate::glam::*;
use rand::Rng;

const CELL_SIZE : f32 = 32.0;
pub const NUM_VERT : i32 = 19;
pub const NUM_HORZ : i32 = 25;

pub fn draw_grid(ctx: &mut Context, width: i32, height: i32) -> GameResult{
    let red = rgba_encoder(255, 0, 0, 1.0);
    let blue = rgba_encoder(0, 0, 255, 1.0);
    let yellow = rgba_encoder(255, 255, 0, 1.0);
    let green = rgba_encoder(0, 255, 0, 1.0);
    let purple = rgba_encoder(128, 0, 128, 1.0);

    let colors = [red, blue, yellow, green, purple];

    let mut rng = rand::thread_rng();

    let mut canvas = Canvas::from_frame(ctx, Color::BLACK);
    let mut rect = Rect::new(0.0, 0.0, CELL_SIZE, CELL_SIZE);

    for i in 5..height {
        let y = i as f32 * CELL_SIZE;
        rect.y = y;
        for j in 0..width {
            let x = j as f32 * CELL_SIZE;
            rect.x = x;
            let col = colors[rng.gen_range(0..=4)];
            canvas.draw(&Quad,
                        DrawParam::new()
                        .dest(rect.point())
                        .scale(rect.size())
                        .color(col));
            
        }
    }
    canvas.finish(ctx)?;
    Ok(())

}

fn rgba_encoder (r: u8, g: u8, b: u8, a: f32) -> Color {
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, a)
}