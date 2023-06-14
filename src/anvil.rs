use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Uniform;
use std::iter;

fn gennonce() -> String {
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
        .take(12)
        .collect();
    hex_chars
}

fn gennoncex() -> String {
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
        .take(24)
        .collect();
    hex_chars
}

fn geniv() -> String {
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
        .take(16)
        .collect();
    hex_chars
}

fn genkey() -> String {
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
        .take(32)
        .collect();
    hex_chars
}

fn main() {
    let nonce = gennonce();
    println!("NONCE: {}", nonce);
    let iv = geniv();
    println!("IV: {}", iv);
    let key = genkey();
    println!("KEY: {}", key);
    let noncex = gennoncex();
    println!("XNONCE: {}", noncex);
}
