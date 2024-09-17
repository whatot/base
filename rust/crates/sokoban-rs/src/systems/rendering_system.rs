use std::{collections::HashMap, time::Duration};

use ggez::graphics::{Canvas, DrawParam, Drawable, InstanceArray, Rect};
use ggez::{
    graphics::{self, Color, Image},
    Context,
};
use glam::vec2;
use specs::{Join, Read, ReadStorage, System};

use crate::{components::RenderableKind, constants, resources::Time};
use crate::{
    components::{Position, Renderable},
    resources::GamePlay,
};

use itertools::Itertools;

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (
        Read<'a, GamePlay>,
        Read<'a, Time>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (gameplay, time, postions, renderables) = data;

        // Clearing the screen (this gives us the background colour)
        let mut canvas = Canvas::from_frame(self.context, Color::from([0.95, 0.95, 0.95, 1.0]));

        // Get all the renderables with their positions and sort by the position z
        // This will allow us to have entities layered visually.
        let mut rendering_data = (&postions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by_key(|&k| k.0.z);
        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

        // Iterate through all pairs of positions & renderables, load the image
        // and draw it at the specified position.
        for (postion, renderable) in rendering_data.iter() {
            // load the image
            let image_path = self.get_image(renderable, time.delta);

            let x = postion.x as f32 * constants::TILE_WIDTH;
            let y = postion.y as f32 * constants::TILE_WIDTH;
            let z = postion.z;

            // draw
            let draw_params = DrawParam::new().dest(vec2(x, y));
            rendering_batches
                .entry(z)
                .or_default()
                .entry(image_path)
                .or_default()
                .push(draw_params);
        }

        // Iterate spritebatches ordered by z and actually render each of them
        for (_z, group) in rendering_batches
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in group {
                let image = Image::from_path(self.context, image_path).expect("expected image");
                let mut instance_array = InstanceArray::new(self.context, image);
                for draw_param in draw_params {
                    instance_array.push(*draw_param);
                }

                instance_array.draw(&mut canvas, DrawParam::new());
            }
        }

        // Render any text
        self.draw_text(&mut canvas, &gameplay.state.to_string(), 525.0, 80.0);
        self.draw_text(&mut canvas, &gameplay.moves_count.to_string(), 525.0, 100.0);
        let fps = format!("FPS: {:.0}", self.context.time.fps());
        self.draw_text(&mut canvas, &fps, 525.0, 120.0);

        // Finally, present the context, this will actually display everything  on the screen.
        let _ = canvas.finish(self.context);
    }
}

impl RenderingSystem<'_> {
    pub fn draw_text(&self, canvas: &mut Canvas, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let color = Color::new(0.0, 0.0, 0.0, 1.0);
        let rect = Rect::new(x, y, 1.0, 1.0);

        text.draw(canvas, DrawParam::new().dest_rect(rect).color(color));
        canvas.set_sampler(graphics::FilterMode::Linear);
    }

    pub fn get_image(&mut self, renderable: &Renderable, delte: Duration) -> String {
        let path_index = match renderable.kind() {
            RenderableKind::Static => {
                // We only have one image, so we just return that
                0
            }
            RenderableKind::Animated => {
                // If we have multiple, we want to select the right one based on the delta time.
                // First we get the delta in milliseconds, we % by 1000 to get the milliseconds
                // only and finally we divide by 250 to get a number between 0 and 4. If it's 4
                // we technically are on the next iteration of the loop (or on 0), but we will let
                // the renderable handle this logic of wrapping frames.
                ((delte.as_millis() % 1000) / 250) as usize
            }
        };

        renderable.path(path_index)
    }
}
