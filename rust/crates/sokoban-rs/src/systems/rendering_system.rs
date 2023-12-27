use ggez::{
    graphics::{self, Color, DrawParam, Image},
    Context,
};
use glam::{vec2, Vec2};
use specs::{Join, Read, ReadStorage, System};

use crate::constants;
use crate::{
    components::{Position, Renderable},
    resources::GamePlay,
};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, GamePlay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, postions, renderables) = data;

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

        // Render any text
        self.draw_text(&gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&gameplay.moves_count.to_string(), 525.0, 100.0);

        // Finally, present the context, this will actually display everything  on the screen.
        graphics::present(self.context).expect("expected to present");
    }
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = Vec2::new(x, y);
        let color = Some(Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = Vec2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }
}
