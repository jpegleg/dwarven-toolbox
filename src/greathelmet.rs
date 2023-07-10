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
    let pass = args[1].clone();
    let spass = String::from(pass);
    let bspass = spass.as_bytes();
    let salt = args[2].clone();
    let ssalt = String::from(salt);
    let bssalt = ssalt.as_bytes();
    let itern = args[3].clone();
    let sitern = String::from(itern);
    let usitern: u32 = sitern.parse().unwrap();
    let mut keyslot = [0u8; 20];

    pbkdf2_hmac::<Sha256>(bspass, bssalt, usitern, &mut keyslot);
    println!("{}", hex::encode(&keyslot));
    keyslot.zeroize();
}
