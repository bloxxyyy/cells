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

pub mod time {
    use std::time::{Instant};

    const WEEK: [&str; 7] = [
        "zondag",
        "maandag",
        "dinsdag",
        "woensdag",
        "donderdag",
        "vrijdag",
        "zaterdag"
    ];

    const TIJDFACTOR: f64 = 10000.0;
    
    #[derive(Copy, Clone)]
    pub struct Time {
        pub now: std::time::Instant,
        pub dagnummer : u32,
        pub game_tijd_dagen: u32,
        pub minutentijd: u32
    }

    impl Time {
        pub fn default() -> Time {
            Time {now: Instant::now(), dagnummer: 0, game_tijd_dagen: 0, minutentijd: 0}
        }

        pub fn update_time(mut self, now: std::time::Instant) -> Time {
            let huidigetijd = now.elapsed().as_millis();

            // in game minuten
            let game_tijd_minuten = irl_naar_gametijd_minuten(huidigetijd);
            let game_tijd_uren = game_tijd_minuten / 60;
            self.game_tijd_dagen = game_tijd_uren / 24;

            self.minutentijd = game_tijd_minuten % (60 * 24);
            
            let game_kloktijd_minuten = game_tijd_minuten % 60;
            let game_kloktijd_uren = game_tijd_uren % 24;
            self.dagnummer = self.game_tijd_dagen % 7;
            let dagnaam = WEEK[self.dagnummer as usize].to_string();

            let datumstring = format!("dag {}: {}. {}:{}", self.game_tijd_dagen, dagnaam, game_kloktijd_uren, game_kloktijd_minuten);
            Time {now: now, dagnummer: self.dagnummer, game_tijd_dagen: self.game_tijd_dagen, minutentijd: self.minutentijd}
        }
    }

    fn irl_naar_gametijd_minuten(milliseconden: u128) -> u32 {
        (milliseconden as f64 * TIJDFACTOR / 1000.0) as u32
    }
}