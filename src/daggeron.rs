use hex;
use num_bigint::{BigUint};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Convert hex data to a BigUint. Usage: daggeron data");
        return;
    }
    let stro = &args[1];
    let bytes: Vec<u8> = match hex::decode(stro) {
        Ok(decoded_bytes) => decoded_bytes,
        Err(_) => {
            eprintln!("Failed to decode the string.");
            return
        }
    };
    let rezint = BigUint::from_bytes_be(&bytes);
    println!("{}", rezint);
}
