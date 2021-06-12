pub mod map_part;
pub use map_part::map_part as part;

use raylib::prelude::*;

pub const MAP_SIZE: u8 = 12;

#[derive(Copy, Clone)]
pub struct Map {
    pub size: [[part::Part; MAP_SIZE as usize]; MAP_SIZE as usize]
}

impl Map {
    pub fn set_map(&mut self) {
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                if y == 3 && x == 3 {
                    self.size[y as usize][x as usize] = part::Part {part_type: part::TileType::Market};
                } else if x == 0 {
                    self.size[y as usize][x as usize] = part::Part {part_type: part::TileType::House};
                } else {
                    self.size[y as usize][x as usize] = part::Part {part_type: part::TileType::Empty};
                }
            }
        }
    }

    pub fn draw_map(self, mut d: raylib::prelude::RaylibDrawHandle) {
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                let dist_x = part::PART_SIZE * x as i32;
                let dist_y = part::PART_SIZE * y as i32;
                match self.size[x as usize][y as usize].part_type {
                    part::TileType::Empty => d.draw_rectangle(dist_x as i32, dist_y as i32, part::PART_SIZE, part::PART_SIZE, Color::BLACK),
                    part::TileType::House => d.draw_rectangle(dist_x as i32, dist_y as i32, part::PART_SIZE, part::PART_SIZE, Color::PINK),
                    part::TileType::Market => d.draw_rectangle(dist_x as i32, dist_y as i32, part::PART_SIZE, part::PART_SIZE, Color::YELLOW)
                }
            }
        }
     }
}