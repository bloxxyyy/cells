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

    const TIJDFACTOR: f64 = 100.0;
    
    #[derive(Copy, Clone)]
    pub struct Time {
        pub now: std::time::Instant,
        pub vorigetijd: u128
    }

    impl Time {
        pub fn default() -> Time {
            Time {now: Instant::now(), vorigetijd: Instant::now().elapsed().as_millis()}
        }

        pub fn update_time(mut self, now: std::time::Instant, vorigetijd: u128) {
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
                self.vorigetijd = huidigetijd;
                let datumstring = format!("dag {}: {}. {}:{}", game_tijd_dagen, dag_van_de_week, game_kloktijd_uren, game_kloktijd_minuten);
                println!("{}", datumstring);
            }
        }
    }

    fn irl_naar_gametijd_minuten(milliseconden: u128) -> u32 {
        (milliseconden as f64 * TIJDFACTOR / 1000.0) as u32
    }
}