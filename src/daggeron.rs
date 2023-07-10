use hex;
use num_bigint::{BigUint};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Convert hex data to a BigUint. Usage: daggeron data");
        return;
    } 
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(stro) = arg.parse::<String>() {
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
    } else {
        println!("No argument provided. \n\nUsage:\n\ndaggeron hexdata\n\n");
    }
}
