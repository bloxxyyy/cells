use crate::human::human::*;
use crate::map::map_part::map_part as tile;
use crate::map::*;
use crate::time::time::*;

use raylib::prelude::*;

pub struct Game {
    pub game_over: bool,
    pub pause: bool,
    pub map: Map,
    pub time: Time,
    pub humans: Humans,
}

pub struct TijdActie {
    pub minuut: u32,
    pub actie: fn(&mut Game),
    pub geldigheids_dagen: u8,
    pub vorige_dag: u32,
}

impl Default for Game {
    fn default() -> Game {
        let game_over = false;
        let pause = false;

        let time = Time::default();

        let mut map = Map {
            size: [[tile::Part {
                part_type: tile::TileType::Empty,
            }; MAP_SIZE as usize]; MAP_SIZE as usize],
        };
        map.set_map();
        let humans = Humans {
            humans: vec![Human {
                home: Vector2 { x: 48.0, y: 64.0 },
                positie: Vector2 { x: 48.0, y: 64.0 },
                places_to_visit: vec![PlaceToVisit {
                    positie: Vector2 { x: 64.0, y: 64.0 },
                    visit_time_minuut: 520,
                    leave_time_minuut: 860,
                }],
            }],
        };

        Game {
            game_over,
            pause,
            map,
            time,
            humans,
        }
    }
}
