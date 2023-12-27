use std::{fmt::Display, time::Duration};

use ggez::event::KeyCode;
use specs::World;

#[derive(Default)]
pub struct InputQueue {
    pub keys_pressed: Vec<KeyCode>,
}

pub enum GamePlayState {
    Playing,
    Won,
}

impl Default for GamePlayState {
    fn default() -> Self {
        Self::Playing
    }
}

impl Display for GamePlayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            GamePlayState::Playing => "Playing",
            GamePlayState::Won => "Won",
        })?;
        Ok(())
    }
}

#[derive(Default)]
pub struct GamePlay {
    pub state: GamePlayState,
    pub moves_count: u32,
}

#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}

pub fn register_resources(world: &mut World) {
    world.insert(InputQueue::default());
    world.insert(GamePlay::default());
    world.insert(Time::default());
}
