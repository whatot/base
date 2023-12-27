use std::collections::HashMap;

use specs::{Join, ReadStorage, System, Write};

use crate::{
    components::{Box, BoxSpot, Position},
    resources::{GamePlay, GamePlayState},
};

pub struct GameplayStateSystem {}

impl<'a> System<'a> for GameplayStateSystem {
    type SystemData = (
        Write<'a, GamePlay>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Box>,
        ReadStorage<'a, BoxSpot>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut gameplay_state, positions, boxes, box_spots) = data;

        // get all boxes indexed by position
        let boxes_by_position: HashMap<(u8, u8), &Box> = (&positions, &boxes)
            .join()
            .map(|t| ((t.0.x, t.0.y), t.1))
            .collect::<HashMap<_, _>>();

        // loop through all box spots and check if there is a corresponding
        // box at that position
        for (_box_spot, position) in (&box_spots, &positions).join() {
            if boxes_by_position.contains_key(&(position.x, position.y)) {
                // continue
            } else {
                gameplay_state.state = GamePlayState::Playing;
                return;
            }
        }

        // If we made it this far, then all box spots have boxes on them, and the
        // game has been won
        gameplay_state.state = GamePlayState::Won;
    }
}
