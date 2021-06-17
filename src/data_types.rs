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
}

pub struct TijdActie<'f> {
    pub minuut: u32,
    pub locatie: Vector2,
    pub geldigheids_dagen: u8,
    pub vorige_dag: u32,
    pub personen: Vec<&'f mut Human>
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

        Game {
            game_over,
            pause,
            map,
            time,
        }
    }
}
