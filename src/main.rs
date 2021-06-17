mod map;
mod time;
mod human;
mod game_functies;
mod data_types;

use game_functies::*;
use data_types::*;
use human::human::*;

use raylib::prelude::*;

fn handleActions(humans: &mut Humans) -> std::vec::Vec<data_types::TijdActie> {
    let mut acties = Vec::new();
    for mut human in humans.humans.iter_mut() {
        let mut h = human;
        acties.push(
            TijdActie {
                minuut: 720,
                locatie: Vector2 {
                    x: 69.0,
                    y: 69.0
                },
                personen: vec![h],
                geldigheids_dagen: 83,
                vorige_dag: u32::MAX
            }
        );
    }
    return acties;
}

fn main() {
    let mut game = Game::default();
    let (mut rl, thread) = raylib::init().size(640, 480).title("Cells").build();

    let mut humans = Humans::default();

    let mut acties = handleActions(&mut humans);

    while !rl.window_should_close() {
        update_game(&mut game, &mut acties);
        draw_game(&mut acties, &game, &mut rl, &thread);
    }
}
