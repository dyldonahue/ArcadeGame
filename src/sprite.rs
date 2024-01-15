use ggez::*; // TODO find exact imports needed

enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub struct Sprite {
    image: graphics::Image,
    pub x: f32
    pub y: f32,
    pub width: i32,
    pub height: i32,
    pub direction: Direction
}

impl Sprite {
    pub fn new(ctx: &mut Context, path: &str, x: f32, y: f32, width: i32, height: i32) -> Sprite {
        let image = graphics::Image::new(ctx, path).unwrap();
        Sprite {
            image,
            x,
            y,
            width,
            height,
            direction: Down  
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let dest_point = graphics::Point2::new(self.x, self.y);
        graphics::draw(ctx, &self.image, (dest_point,))?;
        Ok(())
    }
}

pub fn move_sprite(ctx: &mut Context, x: i32, y: i32) {
    let mut state = &mut State::get_mut(ctx).unwrap();
    state.sprite.x += (x * state.sprite.width as f32) as i32;
    state.sprite.y += (y * state.sprite.height as f32) as i32;
}