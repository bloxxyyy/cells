use crate::human::human::*;
use crate::data_types::*;

use raylib::prelude::*;

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
            (actie_tijd.actie)(game);
        }
    }
}

pub fn draw_game(game: &Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut d = rl.begin_drawing(thread);
    d.clear_background(Color::RAYWHITE);
    game.map.draw_map(&mut d);
    game.humans.draw_humans(&mut d);

    for human in game.humans.humans.iter() {
        let x = human.home.x;
        let y = human.home.y;
        d.draw_circle(x as i32, y as i32, HUMAN_SIZE as f32, Color::DARKPURPLE)
    }
}
