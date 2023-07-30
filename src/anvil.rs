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

fn genkey256() -> String {
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
    let nonce = gennonce();
    println!("NONCE: {}", nonce);
    let iv = geniv();
    println!("IV: {}", iv);
    let key = genkey();
    println!("KEYor256IV: {}", key);
    let noncex = gennoncex();
    println!("XNONCE: {}", noncex);
    let key2 = genkey256();
    println!("256KEY: {}", key2);
}

#[test]
fn test_nonce() {
    let nonce = gennonce();
    let nonce2 = gennonce();

    let num_bytes = nonce.len();
    let byte_distribution = nonce.as_bytes().iter().collect::<std::collections::HashSet<_>>().len() as f64 / num_bytes as f64;

    let threshold = 0.42;
    if byte_distribution < threshold {
        let tolerance = 0.0;
        assert_ne!(nonce2, nonce);
        assert_eq!(threshold, tolerance);
    } else {
        let tolerance = 0.42;
        assert_ne!(nonce2, nonce);
        assert_eq!(threshold, tolerance);
    }

}


#[test]
fn test_iv() {
    let nonce = geniv();
    let nonce2 = geniv();

    let num_bytes = nonce.len();
    let byte_distribution = nonce.as_bytes().iter().collect::<std::collections::HashSet<_>>().len() as f64 / num_bytes as f64;

    let threshold = 0.41;
    if byte_distribution < threshold {
        let tolerance = 0.0;
        assert_ne!(nonce2, nonce);
        assert_eq!(threshold, tolerance);
    } else {
        let tolerance = 0.41;
        assert_ne!(nonce2, nonce);
        assert_eq!(threshold, tolerance);
    }

}

#[test]
fn test_key() {
    let nonce = genkey();
    let nonce2 = genkey();

    let num_bytes = nonce.len();
    let byte_distribution = nonce.as_bytes().iter().collect::<std::collections::HashSet<_>>().len() as f64 / num_bytes as f64;

    let threshold = 0.33;
    if byte_distribution < threshold {
        let tolerance = 0.0;
        assert_ne!(nonce2, nonce);
        assert_eq!(threshold, tolerance);
    } else {
        let tolerance = 0.33;
        assert_ne!(nonce2, nonce);
        assert_eq!(threshold, tolerance);
    }

}

#[test]
fn test_long_nonce() {
    let nonce = gennoncex();
    let nonce2 = gennoncex();

    let num_bytes = nonce.len();
    let byte_distribution = nonce.as_bytes().iter().collect::<std::collections::HashSet<_>>().len() as f64 / num_bytes as f64;

    let threshold = 0.41;
    if byte_distribution < threshold {
        let tolerance = 0.0;
        assert_ne!(nonce2, nonce);
        assert_eq!(threshold, tolerance);
    } else {
        let tolerance = 0.41;
        assert_ne!(nonce2, nonce);
        assert_eq!(threshold, tolerance);
    }

}

#[test]
fn test_256_key() {
    let nonce = genkey256();
    let nonce2 = genkey256();

    let num_bytes = nonce.len();
    let byte_distribution = nonce.as_bytes().iter().collect::<std::collections::HashSet<_>>().len() as f64 / num_bytes as f64;

    let threshold = 0.2;
    if byte_distribution < threshold {
        let tolerance = 0.0;
        assert_ne!(nonce2, nonce);
        assert_eq!(threshold, tolerance);
    } else {
        let tolerance = 0.2;
        assert_ne!(nonce2, nonce);
        assert_eq!(threshold, tolerance);
    }

}
