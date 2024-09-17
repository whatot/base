use std::collections::HashMap;

use specs::{world::Index, Entities, Join, ReadStorage, System, Write, WriteStorage};

use ggez::input::keyboard::KeyCode;

use crate::{
    components::{Immovable, Movable, Player, Position},
    constants,
    events::{EntityMoved, EventKind},
    resources::{EventQueue, GamePlay, InputQueue},
};

pub struct InputSystem {}

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        Write<'a, EventQueue>,
        Write<'a, InputQueue>,
        Write<'a, GamePlay>,
        Entities<'a>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Movable>,
        ReadStorage<'a, Immovable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut event_queue,
            mut input_queue,
            mut gameplay,
            entities,
            mut positions,
            players,
            moveables,
            immovables,
        ) = data;

        let mut to_move = Vec::new();

        for (position, _player) in (&positions, &players).join() {
            // Get the first key pressed
            if let Some(key) = input_queue.keys_pressed.pop() {
                // get all the movables and immovables
                let mov: HashMap<(u8, u8), Index> = (&entities, &moveables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();
                let immov: HashMap<(u8, u8), Index> = (&entities, &immovables, &positions)
                    .join()
                    .map(|t| ((t.2.x, t.2.y), t.0.id()))
                    .collect::<HashMap<_, _>>();

                // Now iterate through current position to the end of the map
                // on the correct axis and check what needs to move.
                let (start, end, is_x) = match key {
                    KeyCode::Up => (position.y, 0, false),
                    KeyCode::Down => (position.y, constants::MAP_HEIGHT, false),
                    KeyCode::Left => (position.x, 0, true),
                    KeyCode::Right => (position.x, constants::MAP_WIDTH, true),
                    _ => continue,
                };

                let range = if start < end {
                    (start..=end).collect::<Vec<_>>()
                } else {
                    (end..=start).rev().collect::<Vec<_>>()
                };

                for x_or_y in range {
                    let pos = if is_x {
                        (x_or_y, position.y)
                    } else {
                        (position.x, x_or_y)
                    };

                    // find a movable
                    // if it exists, we try to move it and continue
                    // if it doesn't exist, we continue and try to find an immovable instead
                    match mov.get(&pos) {
                        Some(id) => to_move.push((key, id.clone())),
                        None => {
                            // find an immovable
                            // if it exists, we need to stop and not move anything
                            // if it doesn't exist, we stop because we found a gap
                            match immov.get(&pos) {
                                Some(_id) => {
                                    to_move.clear();
                                    event_queue.events.push(EventKind::PlayerHitObstacle {});
                                }
                                None => break,
                            }
                        }
                    }
                }
            }
        }

        // We've just moved, so let's increase the number of moves
        if to_move.len() > 0 {
            gameplay.moves_count += 1;
        }

        // Now actually move what needs to be moved
        for (key, id) in to_move {
            let position = positions.get_mut(entities.entity(id));
            if let Some(positiom) = position {
                match key {
                    KeyCode::Up => positiom.y -= 1,
                    KeyCode::Down => positiom.y += 1,
                    KeyCode::Left => positiom.x -= 1,
                    KeyCode::Right => positiom.x += 1,
                    _ => (),
                }
            }

            // Fire an event for the entity that just moved
            event_queue
                .events
                .push(EventKind::EntityMoved(EntityMoved { id }));
        }
    }
}
