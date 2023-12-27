use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, KeyCode, KeyMods},
    Context, GameResult,
};
use input::InputSystem;
use render::RenderingSystem;
use specs::WorldExt;
use specs::{RunNow, World};
use std::path;

mod components;
mod input;
mod render;
mod sk_const;

// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {
    world: World,
}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        {
            // Run input system
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult {
        {
            // render game entities
            let mut rs = RenderingSystem { context };
            rs.run_now(&self.world)
        }

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        println!("Key pressed: {:?}", keycode);

        let mut input_queue = self.world.write_resource::<components::InputQueue>();
        input_queue.keys_pressed.push(keycode);
    }
}

fn main() -> GameResult {
    let mut world = World::new();
    components::register_components(&mut world);
    components::register_resources(&mut world);
    components::initialize_level(&mut world);

    // 2080=32*65,1280=32*40,40/65=0.615
    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(WindowSetup::default().title("Rust Sokoban"))
        .window_mode(
            WindowMode::default().dimensions(sk_const::WINDOWS_WIDTH, sk_const::WINDOWS_HEIGHT),
        )
        .add_resource_path(path::PathBuf::from("./crates/sokoban-rs/resources"));

    let (context, event_loop) = context_builder.build()?;
    // Create the game state
    let game = Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}
