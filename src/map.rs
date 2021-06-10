pub mod map_part;
pub use map_part::map_part as part;

pub const MAP_SIZE: u8 = 32;
pub const TILE_SIZE: f32 = 32.0;

pub struct Map {
    pub size: [[part::Part; MAP_SIZE as usize]; MAP_SIZE as usize]
}

impl Map {
    pub fn set_map(&mut self) {
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                self.size[y as usize][x as usize] = part::Part {test: 1};
            }
        }
    }
}