pub const MAP_SIZE: u8 = 32;
pub const TILE_SIZE: f32 = 32.0;

pub struct Map {
    pub size: [[u8; MAP_SIZE as usize]; MAP_SIZE as usize]
}

impl Map {
    pub fn set_map(&mut self) {
        for y in 0..MAP_SIZE {
            for x in 0..MAP_SIZE {
                self.size[y as usize][x as usize] = 1;
            }
        }
    }
}