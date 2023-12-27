use ggez::{
    graphics::{self, DrawParam, Image},
    Context,
};
use glam::vec2;
use specs::{Join, ReadStorage, System};

use crate::components::{Position, Renderable};
use crate::constants;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);

    fn run(&mut self, data: Self::SystemData) {
        let (postions, renderables) = data;

        // Clearing the screen (this gives us the background colour)
        graphics::clear(self.context, graphics::Color::new(0.95, 0.95, 0.95, 1.0));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&postions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (postion, renderable) in rendering_data.iter() {
            // load the image
            let image = Image::new(self.context, renderable.path.clone()).expect("expected image");
            let x = postion.x as f32 * constants::TILE_WIDTH;
            let y = postion.y as f32 * constants::TILE_WIDTH;

            // draw
            let draw_params = DrawParam::new().dest(vec2(x, y));
            graphics::draw(self.context, &image, draw_params).expect("expected render");
        }

        // Finally, present the context, this will actually display everything  on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}
