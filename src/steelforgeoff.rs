use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
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
        eprintln!("Usage: {} <ciphertext_file> <plaintext_file>", args[0]);
        return Ok(());
    }

    let ciphertext_file_path = &args[1];
    let plaintext_file_path = &args[2];
    let mut ciphertext_file = File::open(ciphertext_file_path)?;
    let mut ciphertext = Vec::new();
    ciphertext_file.read_to_end(&mut ciphertext)?;
    let nonce = chacha20poly1305::XNonce::from_slice(&ciphertext[..24]);
    let tag = GenericArray::clone_from_slice(&ciphertext[24..40]);
    let mut plaintext = ciphertext[40..].to_vec();
    let bnon = hex::encode(&nonce);
    let binding = "00004444".to_owned() + &bnon;
    let salt = binding.as_bytes();
    let strpassword = rpassword::prompt_password("Password: ")?;
    let password = strpassword.as_bytes();
    let hashed_key = hashkey::hash_key(&password, &salt);
    let aead = XChaCha20Poly1305::new(GenericArray::from_slice(&hashed_key));
    aead.decrypt_in_place_detached(&nonce, &[], &mut plaintext, &tag)
        .expect("Error: Decryption failed.");

    let mut plaintext_file = File::create(plaintext_file_path)?;
    plaintext_file.write_all(&plaintext)?;
    println!("Decryption completed successfully.");
    Ok(())
}
