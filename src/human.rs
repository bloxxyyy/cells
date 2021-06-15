pub mod human {
    use raylib::prelude::*;

    pub const HUMAN_SIZE: u8 = 8;

    pub struct Humans {
        pub humans: Vec<Human>
    }

    pub struct Human {
        pub home: Vector2,
        pub positie: Vector2,
        pub places_to_visit: Vec<PlaceToVisit>
    }

    #[derive(Copy, Clone)]
    pub struct PlaceToVisit {
        pub positie: Vector2,
        pub visit_time_minuut: u32,
        pub leave_time_minuut: u32
    }

    impl Human {
        pub fn ga_naar(&mut self, locatie: Vector2) {
            self.positie = locatie.clone();
        }
    }

    impl Humans {
        pub fn draw_humans(&self, d: &mut raylib::prelude::RaylibDrawHandle) {
            for human in self.humans.iter() {
                let x = human.home.x;
                let y = human.home.y;
                d.draw_circle(x as i32, y as i32, HUMAN_SIZE as f32, Color::PINK)
            }
        }
    }
}