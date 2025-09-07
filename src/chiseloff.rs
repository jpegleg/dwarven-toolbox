use std::{str, env};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::convert::TryInto;
use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::Key;
use chacha20poly1305::XNonce;
use pbkdf2::{pbkdf2_hmac};
use sha2::Sha256;
use hex;
use zeroize::Zeroize;

fn decrypt_string(nonce: &[u8], cipher_hex: &str, key: &[u8; 32]) -> Result<String, String> {
    if cipher_hex.len() % 2 == 0 {
        let ciphertext = hex::decode(&cipher_hex).map_err(|_| "Failed to decode ciphertext hex...")?;
        let key = Key::from_slice(key);
        let cipher = chacha20poly1305::XChaCha20Poly1305::new(key);
        let nonce_size = 24;
        if ciphertext.len() < nonce_size {
            return Err("Invalid ciphertext".to_string());
        }

        let (received_nonce, received_ciphertext) = ciphertext.split_at(nonce_size);
        let nonce = if nonce.is_empty() {
            received_nonce
        } else {
            nonce
        };

        let plaintext = cipher
            .decrypt(XNonce::from_slice(nonce), &*received_ciphertext)
            .map_err(|_| "[as-is hex] Decryption failed")?;

        let plaintext = String::from_utf8(plaintext).map_err(|_| "Invalid UTF-8")?;

        Ok(plaintext)


    } else {
        let ciphertext = hex::decode("0".to_owned() + &cipher_hex).map_err(|_| "Failed to decode ciphertext hex modified...")?;
        let key = Key::from_slice(key);
        let cipher = chacha20poly1305::XChaCha20Poly1305::new(key);
        let nonce_size = 24;
        if ciphertext.len() < nonce_size {
            return Err("Invalid ciphertext".to_string());
        }

        let (received_nonce, received_ciphertext) = ciphertext.split_at(nonce_size);
        let nonce = if nonce.is_empty() {
            received_nonce
        } else {
            nonce
        };

        let plaintext = cipher
            .decrypt(XNonce::from_slice(nonce), &*received_ciphertext)
            .map_err(|_| "[adjusted hex prepend 0] Decryption failed")?;

        let plaintext = String::from_utf8(plaintext).map_err(|_| "Invalid UTF-8")?;

        Ok(plaintext)

    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("{{\"ERROR\": \"Wrong number of args. The first arg is the file to decrypt, the second arg is the keypair binary file path, as created from makesigil. The third argument is a salt for the PBKDF2.\"}}");
        std::process::exit(1);
    }
    let data = args[1].clone();
    let sdata = String::from(data);
    let datafile_path = Path::new(&sdata);
    let mut datafile = File::open(&datafile_path).expect("Failed to open the file.");
    let mut databytes = Vec::new();
    datafile.read_to_end(&mut databytes).expect("Failed to read the file.");
    let iciphertext = &databytes[25..];
    let deciphertext = str::from_utf8(&iciphertext).unwrap();
    let inonce: &[u8] = &databytes[0..24];
    let mut nonce: &[u8] = inonce;

    let keyin = args[2].clone();
    let keyfile_path = Path::new(&keyin);
    let mut keyfile = File::open(&keyfile_path).expect("Failed to open the file.");
    let mut keybytes = Vec::new();
    keyfile.read_to_end(&mut keybytes).expect("Failed to read the file.");
    let mut private_key_bytes: &[u8] = &keybytes.as_slice();
    let usitern: u32 = 2100;
    let salt = args[3].clone();
    let ssalt = String::from(salt);
    let bssalt = ssalt.as_bytes();
    let mut keyslot = [0u8; 20];
    pbkdf2_hmac::<Sha256>(&mut private_key_bytes, bssalt, usitern, &mut keyslot);
    let hexkey = hex::encode(&keyslot);
    keyslot.zeroize();
    let izbhex = hexkey.as_bytes();
    let ibhex: &[u8] = &izbhex[0..32];
    let mut bhex: [u8; 32] = ibhex.try_into().unwrap();

    let result = decrypt_string(&mut nonce, &deciphertext, &mut bhex);
    match result {
        Ok(plaintext) => {
            println!("{}", plaintext);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    keybytes.zeroize();
    bhex.zeroize();

}
