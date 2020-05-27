mod graphics;
mod game;

use std::fs::File;
use std::io::Read;

fn main() {
    // get random bits to seed random number generator
    let mut f = File::open("/dev/random").expect("Do I have permissions for this?");
    let mut bits: [u8; 16] = [0; 16];
    match f.read(&mut bits) {
        Ok(b) => {
            if b != 16 { panic!("Wrong number of bytes"); }
        },
        Err(_) => panic!("Read failed")
    }
    let seed1: u64 = ((bits[0] as u64) << 48)
                     | ((bits[1] as u64) << 32)
                     | ((bits[2] as u64) << 16)
                     | (bits[3] as u64);

    let seed2: u64 = ((bits[4] as u64) << 48)
                     | ((bits[5] as u64) << 32)
                     | ((bits[6] as u64) << 16)
                     | (bits[7] as u64);

    let mut g = game::Game::new(seed1, seed2);
    match g.init() {
        Err(_) => panic!("Couldn't load textures."),
        _ => {}
    }
    g.game_loop();
}
