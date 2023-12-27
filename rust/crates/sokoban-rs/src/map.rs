use specs::World;

use crate::components::Position;
use crate::entities;

pub fn load_map(world: &mut World, map_string: String) {
    let rows: Vec<&str> = map_string.trim().split("\n").map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            // Create the position at which to create something on the map
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0, // we will get the z from the factory functions
            };

            // Figure out what object we should create
            match *column {
                "." => entities::create_floor(world, position),
                "W" => {
                    entities::create_floor(world, position);
                    entities::create_wall(world, position);
                }
                "P" => {
                    entities::create_floor(world, position);
                    entities::create_player(world, position);
                }
                "B" => {
                    entities::create_floor(world, position);
                    entities::create_box(world, position);
                }
                "S" => {
                    entities::create_floor(world, position);
                    entities::create_box_spot(world, position);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}
