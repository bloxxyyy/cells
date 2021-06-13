pub mod human {
    use raylib::prelude::*;

    #[derive(Copy, Clone)]
    pub struct Position {
        pub x: i8,
        pub y: i8
    }

    pub const HUMAN_SIZE: u8 = 8;

    #[derive(Copy, Clone)]
    pub struct Humans {
        pub humans: [Human; 1]
    }

    #[derive(Copy, Clone)]
    pub struct Human {
        pub home: Position,
        pub places_to_visit: [PlaceToVisit; 1]
    }

    #[derive(Copy, Clone)]
    pub struct PlaceToVisit {
        pub position: Position,
        pub visit_time_minuut: u32,
        pub leave_time_minuut: u32
    }

    impl Humans {
        pub fn draw_humans(self, mut d: raylib::prelude::RaylibDrawHandle) {
            for human in self.humans.iter() {
                let x = human.home.x;
                let y = human.home.y;
                d.draw_circle(x as i32, y as i32, HUMAN_SIZE as f32, Color::PINK)
            }
        }

    }
}