use raylib::prelude::*;

mod test;
pub use test::*;

struct Game {
    game_over: bool,
    pause: bool,
    map: Map,
}

impl Default for Game {
    fn default() -> Game {
        let game_over = false;
        let pause = false;

        let mut map = Map { size: [[10; test::MAP_SIZE as usize]; test::MAP_SIZE as usize] };
        map.set_map();
        println!("{:?}", map.size);

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

    while !rl.window_should_close() {
        update_game(&mut game, &rl);
        draw_game(&game, &mut rl, &thread);
    }
}

fn init_game(game: &mut Game, rl: &RaylibHandle) {

}

fn update_game(game: &mut Game, rl: &RaylibHandle) {

}

fn draw_game(game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::RAYWHITE);
}