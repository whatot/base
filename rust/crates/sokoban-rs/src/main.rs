use ggez::{
    conf::{WindowMode, WindowSetup},
    event, Context, GameResult,
};
use std::path;

// This struct will hold all our game state
// For now there is nothing to be held, but we'll add
// things shortly.
struct Game {}

// This is the main event loop. ggez tells us to implement
// two things:
// - updating
// - rendering

impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> GameResult {
        // TODO update game logic here
        Ok(())
    }

    fn draw(&mut self, _context: &mut Context) -> GameResult {
        // TODO update game logic here
        Ok(())
    }
}

fn main() -> GameResult {
    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(WindowSetup::default().title("Rust Sokoban"))
        .window_mode(WindowMode::default().dimensions(2000.0, 1500.0))
        .add_resource_path(path::PathBuf::from("./resources"));

    let (context, event_loop) = context_builder.build()?;
    // Create the game state
    let game = Game {};
    // Run the main event loop
    event::run(context, event_loop, game)
}
