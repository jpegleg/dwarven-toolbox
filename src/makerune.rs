use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use base64::{Engine as _};
use ed25519_dalek::{Signature, Signer, Keypair, PUBLIC_KEY_LENGTH};

extern crate base64;
extern crate ed25519_dalek;

use zeroize::Zeroize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("{{\"ERROR\": \"Wrong number of args. The first arg is the file to sign, the second arg is the keypair binary file path, as created from makesigil.\"}}");
        std::process::exit(1);
    }
    let data = args[1].clone();
    let sdata = String::from(data);
    let datafile_path = Path::new(&sdata);
    let mut datafile = File::open(&datafile_path).expect("Failed to open the file.");
    let mut databytes = Vec::new();
    datafile.read_to_end(&mut databytes).expect("Failed to read the file.");
    let keyin = args[2].clone();
    let keyfile_path = Path::new(&keyin);
    let mut keyfile = File::open(&keyfile_path).expect("Failed to open the file.");
    let mut keybytes = Vec::new();
    keyfile.read_to_end(&mut keybytes).expect("Failed to read the file.");
    let keypair: Keypair = Keypair::from_bytes(&keybytes).expect("Failed to read bytes to keypair.");
    let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = keypair.public.to_bytes();
    let pub64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(public_key_bytes); 
    let signature: Signature = keypair.sign(&databytes);
    println!("Public key: {:?}\nSignature: {}", pub64, signature);
    keybytes.zeroize();

}
