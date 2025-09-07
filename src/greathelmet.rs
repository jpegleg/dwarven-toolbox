use std::env;
use pbkdf2::{pbkdf2_hmac};
use sha2::Sha256;
use hex;
use zeroize::Zeroize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("The first argument is a password string, the second is a salt string, and the third is the number of iterations as a u32.\n\nUsage: greathelmet something saltysalt 20\n");
        std::process::exit(1);
    }
    let pass = &args[1];
    let bspass = pass.as_bytes();
    let salt = &args[2];
    let bssalt = salt.as_bytes();
    let itern = &args[3];
    let usitern: u32 = match itern.parse::<u32>() {
        Ok(_) => itern.parse::<u32>().expect("failed to parse u32"),
        _ => {
            eprintln!("Failed to parse u32.");
            return
        }
    };
    let mut keyslot = [0u8; 20];
    pbkdf2_hmac::<Sha256>(bspass, bssalt, usitern, &mut keyslot);
    println!("{}", hex::encode(&keyslot));
    keyslot.zeroize();
}
