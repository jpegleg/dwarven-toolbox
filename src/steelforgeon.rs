use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use argon2::Argon2;
use chacha20poly1305::{
    aead::{AeadInPlace, KeyInit},
    XChaCha20Poly1305,
};
use chacha20poly1305::aead::generic_array::GenericArray;

mod hashkey;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <plaintext_file> <ciphertext_file>", args[0]);
        return Ok(());
    }

    let plaintext_file_path = &args[1];
    let ciphertext_file_path = &args[2];
    let mut plaintext_file = File::open(plaintext_file_path)?;
    let mut plaintext = Vec::new();
    plaintext_file.read_to_end(&mut plaintext)?;
    let mut rng = StdRng::from_entropy();
    let mut nonce = [0u8; 24];
    rng.fill(&mut nonce);
    let bnon = hex::encode(&nonce);
    let binding = "00004444".to_owned() + &bnon;
    let salt = binding.as_bytes();
    let strpassword = rpassword::prompt_password("Password: ")?;
    let password = strpassword.as_bytes();
    let hashed_key = hashkey::a2(&password, &salt);
    let aead = XChaCha20Poly1305::new(GenericArray::from_slice(&hashed_key));
    let mut ciphertext_file = File::create(ciphertext_file_path)?;
    let mut ciphertext = plaintext.to_vec();
    let tag = aead.encrypt_in_place_detached(&nonce.into(), &[], &mut ciphertext)
        .expect("Encryption failed.");
    ciphertext_file.write_all(&nonce)?;
    ciphertext_file.write_all(&tag)?;
    ciphertext_file.write_all(&ciphertext)?;
    println!("Encryption completed successfully.");
    Ok(())
}
