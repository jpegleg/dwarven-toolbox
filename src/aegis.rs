use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Uniform;
use std::iter;

fn genkey512() -> String {
    let mut rng = StdRng::from_entropy();
    let hex_chars: String = iter::repeat(())
        .map(|()| {
            let char_range = Uniform::from(0..16);
            let value = match rng.sample(char_range) {
                0..=9 => (b'0' + rng.sample(Uniform::from(0..10))) as char,
                10..=15 => (b'A' + rng.sample(Uniform::from(0..6))) as char,
                _ => unreachable!(),
            };
            value
        })
        .take(64)
        .collect();
    hex_chars
}

fn main() {
    let key = genkey512();
    println!("{key}");
}
