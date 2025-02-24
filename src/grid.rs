use ggez::graphics::{Color, DrawMode, Mesh, MeshBuilder, Rect};
use ggez::Context;
use ggez::GameResult;

// default size if 800 pixels by 600 pixels
pub const CELL_SIZE: f32 = 32.0;
// const num_horz: i32 = 800 / CELL_SIZE as i32;
// const num_vert: i32 = 600*(2/3) / CELL_SIZE as i32;

const num_horz: i32 = 800;
const num_vert: i32 = 1000;

pub fn draw_grid(ctx: &mut Context, width: i32, height: i32) -> GameResult<Mesh> {

   let mut mesh_builder = MeshBuilder::new();

    for row in 0..num_vert {
        for col in 0..num_horz {
            let x = col as f32 * CELL_SIZE;
            let y = row as f32 * CELL_SIZE;
            let color = get_color(y as usize, height as usize);
            let rect = Rect::new(x, y, CELL_SIZE, CELL_SIZE);
            mesh_builder.rectangle(DrawMode::fill(), rect, color)?;
        }
    }

    let meshdata = mesh_builder.build();
    let mesh = Mesh::from_data(ctx, meshdata);

    Ok(mesh)

}

pub fn dig_cell(ctx: &mut Context, x: f32, y: f32, mesh: &mut Mesh) -> GameResult<Mesh> {
    //use existing mesh to draw a new mesh with a cell removed
    let mut mesh_builder = MeshBuilder::new();
    for row in 0..num_vert {
        for col in 0..num_horz {
            let x_1 = col as f32 * CELL_SIZE;
            let y_1 = row as f32 * CELL_SIZE;
            let color = if x == x_1 && y == y_1 {
                Color::from_rgb(0, 0, 0)
            } else {
                get_color(y_1 as usize, 1000)
            };

            let rect = Rect::new(x, y, CELL_SIZE, CELL_SIZE);
            mesh_builder.rectangle(DrawMode::fill(), rect, color)?;
        }
    }

    let meshdata = mesh_builder.build();
    *mesh = Mesh::from_data(ctx, meshdata);

    Ok(mesh.clone())

}

fn get_color(row: usize, height: usize) -> Color {
    let num_layers = 5;
    let height = height / num_layers;

    if row < height { 
        Color::from_rgb(135, 206, 235) // sky
    }
    else if row < height * 2 {
        Color::from_rgb(200, 180, 140) // sand
    } else if row < height * 3 {
        Color::from_rgb(180, 140, 100) // dirt
    } else if row < height * 4 {
        Color::from_rgb(140, 100, 60) // stone
    } else {
        Color::from_rgb(100, 60, 30) // bedrock
    }
}

fn make_grid_colored(heigh)
