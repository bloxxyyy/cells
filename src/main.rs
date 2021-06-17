mod map;
mod time;
mod human;
mod game_functies;
mod data_types;

use game_functies::*;
use data_types::*;

use raylib::prelude::*;

fn main() {
    let mut game = Game::default();
    let (mut rl, thread) = raylib::init().size(640, 480).title("Cells").build();

/*
* For each places_to_visit in Human, create an TijdActie.
* Zet de actie buiten de TijdActie.
*/

    let mut acties = Vec::new();
    for human in game.humans.humans.iter_mut() {
        acties.push(
            TijdActie {
                minuut: 720,
                locatie: Vector2 {
                    x: 69.0,
                    y: 69.0
                },
                personen: vec![
                    &mut human
                ],
                geldigheids_dagen: 83,
                vorige_dag: u32::MAX
            }
        );
    }

    while !rl.window_should_close() {
        update_game(&mut game, &mut acties);
        draw_game(&game, &mut rl, &thread);
    }
}
