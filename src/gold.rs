use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Uniform;
use std::iter;
use zeroize::Zeroize;

fn gold() {
    let mut rng = StdRng::from_entropy();
    let mut goldchar: String = iter::repeat(())
        .map(|()| {
            let char_range = Uniform::from(32..127);
            rng.sample(char_range) as u8 as char
        })
        .take(96)
        .collect();
    println!("{}", &goldchar);
    goldchar.zeroize();
}

fn main() {
    gold();
}
