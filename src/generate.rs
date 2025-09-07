use rand::Rng;
use std::iter;

const CHARSET: &[u8] = b"ABCDEF0123456789";
const UTFSET: &[u8] = b"1!2@3#4$5%6^7&8*9(0)-_=+qazwsxedcrfvtgbyhnujmik,ol.p;/[']QAZWSXEDCRFVTGBYHNUJMIK<OL>P:?{}";

#[allow(dead_code)]
pub fn hexgen(len: usize) -> String {
    let mut rng = rand::rng();
    let one_char = || CHARSET[rng.random_range(0..CHARSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}

#[allow(dead_code)]
pub fn utfgen(len: usize) -> String {
    let mut rng = rand::rng();
    let one_char = || UTFSET[rng.random_range(0..UTFSET.len())] as char;
    iter::repeat_with(one_char).take(len).collect()
}
