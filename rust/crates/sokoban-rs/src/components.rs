use std::fmt::Display;

use specs::{Component, NullStorage, VecStorage};
use specs::{World, WorldExt};

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Renderable {
    pub path: String,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Wall {}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

#[derive(PartialEq)]
pub enum BoxColour {
    Red,
    Blue,
}

impl Display for BoxColour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BoxColour::Red => "red",
            BoxColour::Blue => "blue",
        })?;
        Ok(())
    }
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Box {
    pub colour: BoxColour,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct BoxSpot {
    pub colour: BoxColour,
}

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Movable;

#[derive(Component, Default)]
#[storage(NullStorage)]
pub struct Immovable;

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Renderable>();
    world.register::<Player>();
    world.register::<Wall>();
    world.register::<Box>();
    world.register::<BoxSpot>();
    world.register::<Movable>();
    world.register::<Immovable>();
}
