use std::env;
use std::iter;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use ed25519_dalek::Keypair;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Uniform;

extern crate base64;
extern crate ed25519_dalek;

use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::Key;
use chacha20poly1305::XNonce;
use pbkdf2::{pbkdf2_hmac};
use sha2::Sha256;
use hex;
use zeroize::Zeroize;

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

fn encrypt_string(nonce: &[u8], file_bytes: &[u8], key: &[u8]) -> Result<Vec<u8>, String> {
    let key = Key::from_slice(key);
    let cipher = chacha20poly1305::XChaCha20Poly1305::new(key);
    let ciphertext = cipher
        .encrypt(XNonce::from_slice(nonce), file_bytes)
        .map_err(|_| "Encryption failed")?;
    let mut output = Vec::new();
    output.extend_from_slice(nonce);
    output.extend_from_slice(&ciphertext);

    Ok(output)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("{{\"ERROR\": \"Wrong number of args. The first arg is the file to encrypt, the second arg is the keypair binary file path, as created from makesigil. The third argument is a salt for the PBKDF2.\"}}");
        std::process::exit(1);
    }
    let data = args[1].clone();
    let sdata = String::from(data);
    let datafile_path = Path::new(&sdata);
    let mut datafile = File::open(&datafile_path).expect("Failed to open the file");
    let mut databytes = Vec::new();
    datafile.read_to_end(&mut databytes).expect("Failed to read the file");
    let keyin = args[2].clone();
    let keyfile_path = Path::new(&keyin);
    let mut keyfile = File::open(&keyfile_path).expect("Failed to open the file");
    let mut keybytes = Vec::new();
    keyfile.read_to_end(&mut keybytes).expect("Failed to read the file");
    let keypair: Keypair = Keypair::from_bytes(&keybytes).expect("Failed to read bytes to keypair.");
    let mut private_key_bytes: [u8; 64] = keypair.to_bytes();

    let usitern: u32 = 2100;
    let salt = args[3].clone();
    let ssalt = String::from(salt);
    let bssalt = ssalt.as_bytes();
    let mut keyslot = [0u8; 20];
    pbkdf2_hmac::<Sha256>(&mut private_key_bytes, bssalt, usitern, &mut keyslot);
    let hexkey = hex::encode(&keyslot);
    keyslot.zeroize();
    let izbhex = hexkey.as_bytes();
    let mut bhex = &izbhex[0..32];

    let nonce = gennoncex();
    let binding = String::from(nonce);
    let snonce = binding.as_bytes();
    let pos = snonce.len();
    let mut buffer = [0u8; 24];
    buffer[..pos].copy_from_slice(snonce);
    
    let result = encrypt_string(&mut buffer, &databytes, &mut bhex);
    match result {
        Ok(ciphertext) => {
            println!("{}{:02}", binding,hex::encode(ciphertext));
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    keybytes.zeroize();

}
