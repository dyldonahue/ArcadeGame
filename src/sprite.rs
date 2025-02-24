use ggez::{Context, GameResult};
use ggez::graphics::{self, Image, DrawParam, Canvas};
use ggez::mint::Point2;


const SPRITE_SCALE: f32 = 0.02;


enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Sprite {
    image: graphics::Image,
    pub x: f32,
    pub y: f32,
    pub width: i32,
    pub height: i32,
    pub direction: Direction
}

impl Sprite {
    pub fn new(ctx: &mut Context, filename: &str, x: f32, y: f32, width: i32, height: i32) -> GameResult<Sprite> {
        let image = Image::from_path(ctx, format!("/{}", filename))?;
     
        Ok(Sprite {
            image,
            x: x,
            y: y,
            width,
            height,
            direction: Direction::Down  
        })
    }

    pub fn draw_sprite(&mut self, _ctx: &mut Context, canvas: &mut Canvas) -> GameResult {

        let scale = SPRITE_SCALE;  
        let dest_point = Point2 { x: self.x, y: self.y };
        let draw_param = DrawParam::default()
            .dest(dest_point)
            .scale([scale, scale]);

        canvas.draw(&self.image, draw_param);

        Ok(())
    }

    pub fn move_sprite(&mut self, ctx: &mut Context, x: f32, y: f32) {

        self.x += x;
        self.y += y;

        
    }
}

