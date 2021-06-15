pub mod map_part {
    pub const PART_SIZE: i32 = 32;

    #[derive(Copy, Clone)]
    pub struct Part {
        pub part_type: TileType
    }

    #[derive(Copy, Clone)]
    pub enum TileType {
        House,
        Market,
        Empty
    }
}