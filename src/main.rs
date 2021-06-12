use raylib::prelude::*;
use std::time::{Duration, Instant};

mod map;
use map::map_part::map_part as tile;
use map::Map;
const TIJDFACTOR: f64 = 10000.0;

struct Game {
    game_over: bool,
    pause: bool,
    map: Map,
}

struct TijdActie {
    minuut: u32,
    actie: fn(&mut Game),
    geldigheids_dagen: u8,
    vorige_dag: u32
}

// 
// 0: Zondag
// 0: Maandag

impl Default for Game {
    fn default() -> Game {
        let game_over = false;
        let pause = false;

        let mut map = Map {
            size: [[tile::Part {
                part_type: tile::TileType::Empty,
            }; map::MAP_SIZE as usize]; map::MAP_SIZE as usize],
        };
        map.set_map();

        Game {
            game_over,
            pause,
            map,
        }
    }
}

const WEEK: [&str; 7] = [
    "zondag",
    "maandag",
    "dinsdag",
    "woensdag",
    "donderdag",
    "vrijdag",
    "zaterdag"
];       

fn main() {
    let mut game = Game::default();
    let (mut rl, thread) = raylib::init().size(640, 480).title("Cells").build();
    init_game(&mut game, &rl);

    let now = Instant::now();

    /*
    uur / milliseconde = factor

    uur
    ------
    factor * milliseconde

    milliseconde = uur / factor

    

    minuut gametime
    ------------------
    seconde real life time * TIJDFACTOR

    gametime = seconde real life * TIJDFACTOR



    1 seconde per minuut
    1000 ms per minuut
    1000 ms / (1/60) minuut

    1000 * 60 ms/min
    60 * 60 * 1000 ms / uur

    3600 seconde per uur
    3600000 ms per uur

    1 / ()
    */

    // minuut gametime per seconde real life time;


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
        let huidigetijd = now.elapsed().as_millis();

        // in game minuten
        let game_tijd_minuten = irl_naar_gametijd_minuten(huidigetijd);
        let game_tijd_uren = game_tijd_minuten / 60;
        let game_tijd_dagen = game_tijd_uren / 24;

        let minutentijd = game_tijd_minuten % (60 * 24);
        
        let game_kloktijd_minuten = game_tijd_minuten % 60;
        let game_kloktijd_uren = game_tijd_uren % 24;
        let dagnummer = game_tijd_dagen % 7;
        let dagnaam = WEEK[dagnummer as usize].to_string();

        let datumstring = format!("dag {}: {}. {}:{}", game_tijd_dagen, dagnaam, game_kloktijd_uren, game_kloktijd_minuten);

        for actie_tijd in &mut acties {
            // 01111111 compleet gevulde week
            //  zmdwdvz
            // 11111000 week [dagnummer] naar links geschoven
            // 01000000 getal 64. 2e bit is 1, omdat we de 2e bit checken

            // 01000000
            if actie_tijd.geldigheids_dagen << dagnummer & 64 == 64
                && actie_tijd.vorige_dag != game_tijd_dagen
                && minutentijd >= actie_tijd.minuut
            {
                actie_tijd.vorige_dag = game_tijd_dagen;

                (actie_tijd.actie)(&mut game);
            }
        }


        update_game(&mut game, &rl);
        draw_game(&game, &mut rl, &thread);
    }
}

fn irl_naar_gametijd_minuten(milliseconden: u128) -> u32 {
    (milliseconden as f64 * TIJDFACTOR / 1000.0) as u32
}

fn init_game(game: &mut Game, rl: &RaylibHandle) {}

fn update_game(game: &mut Game, rl: &RaylibHandle) {}

fn draw_game(game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::RAYWHITE);
    game.map.draw_map(d);
    //d.draw_circle(32, 32, 8.0, Color::GRAY);
}
