pub mod human {
    use raylib::prelude::*;

    pub const HUMAN_SIZE: u8 = 8;

    pub struct Humans {
        pub humans: Vec<Human>
    }

    impl Default for Humans {
        fn default() -> Humans {
            let humans = Humans {
                humans: vec![Human {
                    home: Vector2 { x: 48.0, y: 64.0 },
                    positie: Vector2 { x: 48.0, y: 64.0 },
                }],
            };
            humans
        }
    }

    pub struct Human {
        pub home: Vector2,
        pub positie: Vector2
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