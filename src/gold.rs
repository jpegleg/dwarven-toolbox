use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Uniform;
use std::iter;

fn gold() {
    let mut rng = StdRng::from_entropy();
    let goldchar: String = iter::repeat(())
        .map(|()| {
            let char_range = Uniform::from(32..127);
            rng.sample(char_range) as u8 as char
        })
        .take(96)
        .collect();
    println!("{}", goldchar);
}

fn main() {
    gold();
}
