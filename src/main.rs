use raylib::prelude::*;

mod map;
use map::map_part::map_part as tile;
use map::Map;

mod time;
use time::time::*;

struct Game {
    game_over: bool,
    pause: bool,
    map: Map,
    time: Time,
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

        Game {
            game_over,
            pause,
            map,
            time
        }
    }
}

fn main() {
    let mut game = Game::default();
    let (mut rl, thread) = raylib::init().size(640, 480).title("Cells").build();
    while !rl.window_should_close() {
        update_game(&mut game, &rl);
        draw_game(&game, &mut rl, &thread);
    }
}

fn update_game(game: &mut Game, rl: &RaylibHandle) {
    game.time.update_time(game.time.now, game.time.vorigetijd);
}

fn draw_game(game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::RAYWHITE);
    game.map.draw_map(d);
    //d.draw_circle(32, 32, 8.0, Color::GRAY);
}
