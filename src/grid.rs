use ggez::graphics::*;
use ggez::GameResult;
use ggez::Context;
use ggez::mint;
use crate::glam::*;

const CELL_SIZE : f32 = 16.0;

trait Grid {
    fn grid(&mut self, width: i32, height: i32, color: Color) -> &mut Self;
}

impl Grid for MeshBuilder {
    fn grid(&mut self, width: i32, height: i32, color: Color) -> &mut Self {
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
        self
    }
}

pub fn create_grid(ctx: &mut Context, width: i32, height: i32, color: Color) -> Mesh {
    let mut mesh_builder = MeshBuilder::new();
    mesh_builder.grid(width, height, color);
    let mesh_data = mesh_builder.build();
    let mesh = Mesh::from_data(&ctx.gfx, mesh_data);
    mesh

}

pub fn batch_grid(ctx: &mut Context) -> InstanceArray {
    
    let total_size = ctx.gfx.drawable_size();
    let xtems = (total_size.0 / 16.0) as usize;
        let ytems = (total_size.1 / 16.0) as usize;
        let mut mesh_batch = InstanceArray::new(ctx, None);
        mesh_batch.resize(ctx, xtems * ytems);

        mesh_batch.set((0..xtems).flat_map(|x| {
            (0..ytems).map(move |y| {
                let x = x as f32;
                let y = y as f32;
        
                DrawParam::new()
                    .dest(Vec2::new(x * 16.0, y * 16.0))
            })
        }));
        mesh_batch
}