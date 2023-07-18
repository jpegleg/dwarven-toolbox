use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use base64::{Engine as _};
use ed25519_dalek::{Verifier, PublicKey, Signature};

extern crate base64;
extern crate ed25519_dalek;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("{{\"ERROR\": \"Wrong number of args. The first arg is the file to verify, the second arg is the public key, and the third arg is the signature.\"}}");
        std::process::exit(1);
    }
    let data = args[1].clone();
    let sdata = String::from(data);
    let datafile_path = Path::new(&sdata);
    let mut datafile = File::open(&datafile_path).expect("Failed to open the file.");
    let mut databytes = Vec::new();
    datafile.read_to_end(&mut databytes).expect("Failed to read the file.");
    let keyin = args[2].clone();
    let signin = args[3].clone();
    let upubkey = base64::engine::general_purpose::STANDARD_NO_PAD.decode(keyin).unwrap();
    let pubkey = PublicKey::from_bytes(&upubkey).unwrap();
    let sigproc = signin.as_bytes();
    let dehexsign = hex::decode(sigproc).unwrap();
    let esigna: Signature = Signature::from_bytes(&dehexsign).unwrap();
    let veri = pubkey.verify(&databytes, &esigna).is_ok();
    println!("Verification: {}", veri);
}
