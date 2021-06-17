use crate::human::human::*;
use crate::data_types::*;

use raylib::prelude::*;

// removed the &mut here.. prob a bad idea lol
pub fn update_game(game: &mut Game, acties: &mut Vec<TijdActie>) {
    game.time = game.time.update_time(game.time.now); // guess we will update time by reassigning it?

    for actie_tijd in acties.iter_mut() {
        // 01111111 compleet gevulde week
        // _zmdwdvz
        // 11111000 week [dagnummer] naar links geschoven
        // 01000000 getal 64. 2e bit is 1, omdat we de 2e bit checken

        // 01000000
        if actie_tijd.geldigheids_dagen << game.time.dagnummer & 64 == 64
            && actie_tijd.vorige_dag != game.time.game_tijd_dagen
            && game.time.minutentijd >= actie_tijd.minuut
        {
            actie_tijd.vorige_dag = game.time.game_tijd_dagen;
            
        }
    }
}

pub fn draw_game(acties: &mut Vec<TijdActie>, game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::RAYWHITE);
    game.map.draw_map(&mut d);

    for actie in acties.iter_mut() {
        for human in actie.personen.iter_mut() {
            let x = human.home.x;
            let y = human.home.y;
            d.draw_circle(x as i32, y as i32, HUMAN_SIZE as f32, Color::DARKPURPLE)
        }
    }
}
