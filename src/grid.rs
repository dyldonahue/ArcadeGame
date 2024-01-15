use ggez::graphics::*;
use ggez::GameResult;
use ggez::context::Has;
use ggez::mint;

const CELL_SIZE : f32 = 20.0;

trait Grid {
    fn grid(&mut self, width: i32, height: i32, color: Color) -> GameResult<&mut Self>;
}

impl Grid for MeshBuilder {
    fn grid(&mut self, width: i32, height: i32, color: Color) -> GameResult<&mut Self> {
        let mut lines = vec![];

        // horizontals
        for i in 0..=height {
            let y = i as f32 * CELL_SIZE;
            lines.push(mint::Point2 { x: 0.0, y: y });
            lines.push(mint::Point2 { x: width as f32 * CELL_SIZE, y: y });
        }
        
        // verticals
        for i in 0..=width {
            let x = i as f32 * CELL_SIZE;
            lines.push(mint::Point2 { x: x, y: 0.0 }); 
            lines.push(mint::Point2 { x: x, y: height as f32 * CELL_SIZE });
        }

        let err = self.polyline(DrawMode::stroke(1.0), &lines, color); // TODO handle error (compiler didn't like when i ignored it)

        Ok(self)
    }
}

pub fn create_grid(gfx: &impl Has<GraphicsContext>, width: i32, height: i32, color: Color) -> GameResult<Mesh> {
    let mut mesh_builder = MeshBuilder::new();
    mesh_builder.grid(width, height, color)?;
    let mesh_data = mesh_builder.build();
    let mesh = Mesh::from_data(gfx, mesh_data);
    Ok(mesh)
}