use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Uniform;
use std::iter;
use zeroize::Zeroize;

fn silver() {
    let mut rng = StdRng::from_entropy();
    let mut silverchar: String = iter::repeat(())
        .map(|()| {
            let char_range = Uniform::from(32..127);
            rng.sample(char_range) as u8 as char
        })
        .take(64)
        .collect();
    println!("{}", &silverchar);
    silverchar.zeroize();
}

fn main() {
    silver();
}
