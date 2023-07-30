use std::env;
use rand::rngs::OsRng;
use base64::{Engine as _};
use ed25519_dalek::{Signature, Signer, Keypair, PUBLIC_KEY_LENGTH};

extern crate base64;
extern crate rand;
extern crate ed25519_dalek;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Pass one argument which is to be signed ephemerally. If there is whitespace in the input, surround in doublequotes.\n\nUsage: amuleton mydata");
        std::process::exit(1);
    }

    let input = args[1].clone();
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = keypair.public.to_bytes();
    let pub64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(public_key_bytes); 
    let message: &[u8] = input.as_bytes();
    let signature: Signature = keypair.sign(message);
    println!("Public key: {:?}\nSignature: {}", pub64, signature);
}

#[test]
fn test() {
    let input = "The great and powerful dwarven amulet!";
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = keypair.public.to_bytes();
    let _ = base64::engine::general_purpose::STANDARD_NO_PAD.encode(&public_key_bytes);
    let message: &[u8] = input.as_bytes();
    let signature: Signature = keypair.sign(&message);
    let mut csprng2 = OsRng{};
    let keypair2: Keypair = Keypair::generate(&mut csprng2);
    let public_key_bytes2: [u8; PUBLIC_KEY_LENGTH] = keypair2.public.to_bytes();
    let _ = base64::engine::general_purpose::STANDARD_NO_PAD.encode(&public_key_bytes2);
    let signature2: Signature = keypair2.sign(message);
    assert_ne!(signature2, signature);
}
