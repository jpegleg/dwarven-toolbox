use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use blake3::Hasher;
use chacha20poly1305::{
    aead::{AeadInPlace, KeyInit},
    XChaCha20Poly1305,
};
use chacha20poly1305::aead::generic_array::GenericArray;

mod hashkey;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <ciphertext_file> <key_file> <plaintext_file>", args[0]);
        return Ok(());
    }

    let ciphertext_file_path = &args[1];
    let key_file_path = &args[2];
    let plaintext_file_path = &args[3];
    let mut ciphertext_file = File::open(ciphertext_file_path)?;
    let mut ciphertext = Vec::new();
    ciphertext_file.read_to_end(&mut ciphertext)?;
    let mut key_file = File::open(key_file_path)?;
    let mut key_data = Vec::new();
    key_file.read_to_end(&mut key_data)?;
    let hashed_key = hashkey::b3(&key_data);
    let aead = XChaCha20Poly1305::new(GenericArray::from_slice(&hashed_key));
    let nonce = chacha20poly1305::XNonce::from_slice(&ciphertext[..24]);
    let tag = GenericArray::clone_from_slice(&ciphertext[24..40]);
    let mut plaintext = ciphertext[40..].to_vec();
    aead.decrypt_in_place_detached(&nonce, &[], &mut plaintext, &tag)
        .expect("Error: Decryption failed.");

    let mut plaintext_file = File::create(plaintext_file_path)?;
    plaintext_file.write_all(&plaintext)?;
    println!("Decryption completed successfully.");
    Ok(())
}
