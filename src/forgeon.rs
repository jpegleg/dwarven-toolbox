use std::env;
use std::iter;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use ed25519_dalek::Keypair;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Uniform;

extern crate base64;
extern crate ed25519_dalek;

use chacha20poly1305::{
    aead::{stream, KeyInit},
    XChaCha20Poly1305,
};
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
        .take(19)
        .collect();
    hex_chars
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("{{\"ERROR\": \"Wrong number of args. The first arg is the file to encrypt, the second arg is the keypair binary file path, as created from makesigil. The third argument is destination file path for the ciphertext.\"}}");
        std::process::exit(1);
    }
    let data = args[1].clone();
    let ciph = args[3].clone();
    let sdata = String::from(data);
    let scipher = String::from(ciph);
    let datafile_path = Path::new(&sdata);
    let destfile_path = Path::new(&scipher);
    let _ = File::open(&datafile_path).expect("Failed to open the file");
    let keyin = args[2].clone();
    let keyfile_path = Path::new(&keyin);
    let mut keyfile = File::open(&keyfile_path).expect("Failed to open the file");
    let mut keybytes = Vec::new();
    keyfile.read_to_end(&mut keybytes).expect("Failed to read the file");
    let keypair: Keypair = Keypair::from_bytes(&keybytes).expect("Failed to read bytes to keypair.");
    let mut private_key_bytes: [u8; 64] = keypair.to_bytes();

    let usitern: u32 = 2100;
    let ssalt = "e7922c935f16707-H.+";
    let bssalt = ssalt.as_bytes();
    let mut keyslot = [0u8; 20];
    pbkdf2_hmac::<Sha256>(&mut private_key_bytes, bssalt, usitern, &mut keyslot);
    let hexkey = hex::encode(&keyslot);
    keyslot.zeroize();
    let izbhex = hexkey.as_bytes();
    let bhex = &izbhex[0..32];

    let nonce = gennoncex();
    let binding = String::from(&nonce);
    let snonce = &binding.as_bytes();
    let pos = snonce.len();
    let mut nonbuf = [0u8; 19];
    nonbuf[..pos].copy_from_slice(snonce);
    println!("NONCE: {}", binding);
   
    const BUFLEN: usize = 4096;
    let mut buffer = [0u8; BUFLEN];

    let mut source_file = File::open(datafile_path).expect("Failed to open source file.");
    let mut destfile = File::create(destfile_path).expect("Failed to open destination file.");

    let aead = XChaCha20Poly1305::new(bhex.as_ref().into());
    let mut streaming = stream::EncryptorBE32::from_aead(aead, nonbuf.as_ref().into());


    loop {
        let read_count = source_file.read(&mut buffer).unwrap();

        if read_count == BUFLEN {
            let ciphertext = streaming
                .encrypt_next(buffer.as_slice())
                .map_err(|_err| "ERROR encrypting large file.");
            let writeciph = &ciphertext.unwrap();
            let _ = destfile.write(&writeciph);
        } else {
            let ciphertext = streaming
                .encrypt_last(&buffer[..read_count])
                .map_err(|_err| "ERROR encrypting large file.");
            let writeciph = &ciphertext.unwrap();
            let _ = destfile.write(&writeciph);
            break;
        }
    }

    keybytes.zeroize();

}
