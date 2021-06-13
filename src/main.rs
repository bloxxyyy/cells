use raylib::prelude::*;

mod map;
use map::map_part::map_part as tile;
use map::Map;

mod time;
use time::time::*;

mod human;
use human::human::*;

struct Game {
    game_over: bool,
    pause: bool,
    map: Map,
    time: Time,
    humans: human::human::Humans
}

#[derive(Copy, Clone)]
struct TijdActie {
    minuut: u32,
    actie: fn(&mut Game),
    geldigheids_dagen: u8,
    vorige_dag: u32
}

impl Default for Game {

    fn default() -> Game {
        let game_over = false;
        let pause = false;

        let time = Time::default();

        let mut map = Map {
            size: [[tile::Part {
                part_type: tile::TileType::Empty,
            }; map::MAP_SIZE as usize]; map::MAP_SIZE as usize],
        };
        
        map.set_map();
        let humans = human::human::Humans {
             humans: [
                Human {
                    home: Position {x: 48, y: 64},
                    places_to_visit: [
                        PlaceToVisit {
                        position: Position {x: 64, y: 64},
                        visit_time_minuut: 520,
                        leave_time_minuut: 860
                        }
                    ]
                }
            ]
        };

        Game {
            game_over,
            pause,
            map,
            time,
            humans
        }
    }
}       

fn main() {
    let mut game = Game::default();
    let (mut rl, thread) = raylib::init().size(640, 480).title("Cells").build();

    let mut acties: [TijdActie; 2] = [
        TijdActie {
            minuut: 720,
            actie: |game: &mut Game| {
                // ga naar supermarkt
                println!("supermarkt time yay");
            },
            geldigheids_dagen: 83,
            vorige_dag: u32::MAX
        },
        TijdActie {
            minuut: 540,
            actie: |game: &mut Game| {
                // ga naar supermarkt
                println!("school time cool time");
            },
            geldigheids_dagen: 62,
            vorige_dag: u32::MAX
        }     
    ];

    while !rl.window_should_close() {
        update_game(&mut game, &rl, &mut acties);
        draw_game(&game, &mut rl, &thread);
    }
}

fn update_game(game: &mut Game, rl: &RaylibHandle, acties: &mut [TijdActie; 2]) {
    game.time = game.time.update_time(game.time.now); // guess we will update time by reasigning it?

    for actie_tijd in acties.iter_mut() {
        // 01111111 compleet gevulde week
        //  zmdwdvz
        // 11111000 week [dagnummer] naar links geschoven
        // 01000000 getal 64. 2e bit is 1, omdat we de 2e bit checken

        // 01000000
        if actie_tijd.geldigheids_dagen << game.time.dagnummer & 64 == 64
            && actie_tijd.vorige_dag != game.time.game_tijd_dagen
            && game.time.minutentijd >= actie_tijd.minuut
        {
            actie_tijd.vorige_dag = game.time.game_tijd_dagen;
            (actie_tijd.actie)(game);
        }
    }
}

fn draw_game(game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::RAYWHITE);
    d = game.map.draw_map(d);
    //game.humans.draw_humans(d);

    for human in game.humans.humans.iter() {
        let x = human.home.x;
        let y = human.home.y;
        println!("{} {}", x, y);
        d.draw_circle(x as i32, y as i32, HUMAN_SIZE as f32, Color::DARKPURPLE)
    }
}
