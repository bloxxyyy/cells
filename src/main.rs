use raylib::prelude::*;
use std::time::{Duration, Instant};

mod map;
use map::map_part::map_part as tile;
use map::Map;
const TIJDFACTOR: f64 = 100.0;

struct Game {
    game_over: bool,
    pause: bool,
    map: Map,
}

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

fn main() {
    let mut game = Game::default();
    let (mut rl, thread) = raylib::init().size(640, 480).title("Cells").build();
    init_game(&mut game, &rl);

    let now = Instant::now();

    let mut vorigetijd: u128 = now.elapsed().as_millis();
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




    while !rl.window_should_close() {
        let huidigetijd = now.elapsed().as_millis();
        let tijdsverschil = huidigetijd - vorigetijd;
        // in game minuten
        let game_tijdsverschil = irl_naar_gametijd_minuten(tijdsverschil);
        let game_tijd_minuten = irl_naar_gametijd_minuten(huidigetijd);
        let game_tijd_uren = game_tijd_minuten / 60;
        let game_tijd_dagen = game_tijd_uren / 24;
        
        let game_kloktijd_minuten = game_tijd_minuten % 60;
        let game_kloktijd_uren = game_tijd_uren % 24;
        let dag_van_de_week = [
            "zondag",
            "maandag",
            "dinsdag",
            "woensdag",
            "donderdag",
            "vrijdag",
            "zaterdag"
        ][(game_tijd_dagen % 7) as usize].to_string();

        if game_tijdsverschil >= 1 {
            vorigetijd = huidigetijd;
            let datumstring = format!("dag {}: {}. {}:{}", game_tijd_dagen, dag_van_de_week, game_kloktijd_uren, game_kloktijd_minuten);
            println!("{}", datumstring);
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
