#![deny(clippy::all)]

mod editor;
mod prelude;
use editor::Editor;

fn main() {
    Editor::new().unwrap().run();
}
