use zeroize::Zeroize;

#[path = "./generate.rs"]
mod generate;
use generate::hexgen;

fn gennonce() -> String {
  hexgen(12)
}

fn gennoncex() -> String {
  hexgen(24)
}

fn geniv() -> String {
  hexgen(16)
}

fn genkey() -> String {
  hexgen(32)
}

// the usage of this can be for 256 bit keys (hex decode before use)
fn genkey512() -> String {
  hexgen(64)
}

fn main() {
    let mut nonce = gennonce();
    println!("NONCE: {}", nonce);
    nonce.zeroize();
    let mut iv = geniv();
    println!("IV: {}", iv);
    iv.zeroize();
    let mut key = genkey();
    println!("KEYor256IV: {}", key);
    key.zeroize();
    let mut noncex = gennoncex();
    println!("XNONCE: {}", noncex);
    noncex.zeroize();
    let mutkey2 = genkey512();
    println!("512KEY: {}", key2);
    key2.zeroize();
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
fn test_512_key() {
    let nonce = genkey512();
    let nonce2 = genkey512();

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
