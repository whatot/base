use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self},
    input::keyboard::KeyInput,
    Context, GameError, GameResult,
};
use resources::Time;
use specs::{RunNow, World, WorldExt};
use std::path;
use systems::{EventSystem, GameplayStateSystem, InputSystem, RenderingSystem};

mod audio;
mod components;
mod constants;
mod entities;
mod events;
mod map;
mod resources;
mod systems;

// This struct will hold all our game state
struct Game {
    world: World,
}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, context: &mut Context) -> GameResult {
        {
            // Run input system
            let mut is = InputSystem {};
            is.run_now(&self.world);
        }

        {
            // Run gameplay state system
            let mut gss = GameplayStateSystem {};
            gss.run_now(&self.world);
        }

        {
            // Get and update time resource
            let mut time = self.world.write_resource::<Time>();
            time.delta += context.time.delta();
        }

        {
            // Run event system
            let mut es = EventSystem { context };
            es.run_now(&self.world)
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
        key_input: KeyInput,
        _repeat: bool,
    ) -> Result<(), GameError> {
        println!("Key pressed: {:?}", key_input);

        let mut input_queue = self.world.write_resource::<resources::InputQueue>();
        if let Some(key) = key_input.keycode {
            input_queue.keys_pressed.push(key);
        }
        Ok(())
    }
}

pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ";
    map::load_map(world, MAP.to_string());
}

fn main() -> GameResult {
    let mut world = World::new();
    components::register_components(&mut world);
    resources::register_resources(&mut world);
    initialize_level(&mut world);

    // 2080=32*65,1280=32*40,40/65=0.615
    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(WindowSetup::default().title("Rust Sokoban"))
        .window_mode(
            WindowMode::default().dimensions(constants::WINDOWS_WIDTH, constants::WINDOWS_HEIGHT),
        )
        .add_resource_path(path::PathBuf::from("./crates/sokoban-rs/resources"));

    let (mut context, event_loop) = context_builder.build()?;
    audio::initialize_sounds(&mut world, &mut context);

    // Create the game state
    let game = Game { world };
    // Run the main event loop
    event::run(context, event_loop, game)
}
