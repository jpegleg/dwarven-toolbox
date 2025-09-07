use blake2::{Blake2b512, Digest};
use sha2::Sha256;
use sha3::{Sha3_256,Sha3_384};
use std::env;
extern crate blake2;
extern crate digest;
extern crate sha2;
extern crate sha3;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Pass one argument which is to be hashed. If there is whitespace in the input, surround in doublequotes.\n\nUsage: crown mydata");
        std::process::exit(1);
    }

    let stro = &args[1];
    let data = stro.as_bytes();
    let mut hasher = blake3::Hasher::new();
    hasher.update(data);
    let blake3 = hasher.finalize();
    println!("BLAKE3: {}", blake3);
    let mut hasher = Blake2b512::new();
    hasher.update(data);
    let blake2b512 = hasher.finalize();
    println!("BLAKE2B-512: {:x}", blake2b512);
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    let sha3256 = hasher.finalize();
    println!("SHA3-256: {:x}", sha3256);
    let mut hasher = Sha3_384::new();
    hasher.update(data);
    let sha3384 = hasher.finalize();
    println!("SHA3-384: {:x}", sha3384);
    let mut hasher = Sha256::new();
    hasher.update(data);
    let sha2 = hasher.finalize();
    println!("SHA2: {:x}", sha2);
}
