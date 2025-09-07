use rand::Rng;
use std::iter;

const CHARSET: &[u8] = b"ABCDEF0123456789";

pub fn hexgen(len: usize) -> String {
    let mut rng = rand::rng();
    let one_char = || CHARSET[rng.random_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}
