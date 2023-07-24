use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use argon2::Argon2;
use chacha20poly1305::{
    aead::{AeadInPlace, KeyInit},
    XChaCha20Poly1305,
};
use chacha20poly1305::aead::generic_array::GenericArray;
use rand::Rng;
use rand::SeedableRng;
use rand::rngs::StdRng;
use hex;
use uuid::Uuid;

mod keyhash;

fn forge<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut rng = StdRng::from_entropy();
    let mut nonce = [0u8; 24];
    rng.fill(&mut nonce);
    let bnon = hex::encode(&nonce);
    let binding = "00000000".to_owned() + &bnon;
    let salt = binding.as_bytes();
    let strpassword = rpassword::prompt_password("Password: ")?;
    let password = strpassword.as_bytes();
    let hashed_key = hashkey::hash_key(&password, &salt);
    let aead = XChaCha20Poly1305::new(GenericArray::from_slice(&hashed_key));
    let mut ciphertext = Vec::new();
    let _ = input.read_to_end(&mut ciphertext);
    let tag = aead.encrypt_in_place_detached(&nonce.into(), &[], &mut ciphertext)
        .expect("Encryption failed.");
    output.write_all(&nonce)?;
    output.write_all(&tag)?;
    output.write_all(&ciphertext)?;
    Ok(())
}

fn unforge<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut ciphertext = Vec::new();
    input.read_to_end(&mut ciphertext)?;
    let nonce = chacha20poly1305::XNonce::from_slice(&ciphertext[..24]);
    let tag = GenericArray::clone_from_slice(&ciphertext[24..40]);
    let mut plaintext = ciphertext[40..].to_vec();
    let bnon = hex::encode(&nonce);
    let binding = "00000000".to_owned() + &bnon;
    let salt = binding.as_bytes();
    let strpassword = rpassword::prompt_password("Password: ")?;
    let password = strpassword.as_bytes();
    let hashed_key = hashkey::hash_key(&password, &salt);
    let aead = XChaCha20Poly1305::new(GenericArray::from_slice(&hashed_key));
    aead.decrypt_in_place_detached(&nonce, &[], &mut plaintext, &tag)
        .expect("Error: Decryption failed.");
    output.write_all(&plaintext)?;
    Ok(())
}


fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path> [-d]", args[0]);
        return Ok(());
    }

    let file_path = &args[args.len() - 1];
    let decrypt_flag = args.iter().any(|arg| arg == "-d");

    if decrypt_flag {
        let mut input_file = File::open(file_path)?;
        let chaos = Uuid::new_v4().to_string();
        let temp_file_path = format!("{}.tmp_{}", file_path, chaos);
        let mut output_file = File::create(&temp_file_path)?;
        unforge(&mut input_file, &mut output_file)?;
        std::fs::rename(&temp_file_path, file_path)?;
        println!("Data decrypted and written to file: {}", file_path);
    } else {
        let mut input_file = File::open(file_path)?;
        let chaos = Uuid::new_v4().to_string();
        let temp_file_path = format!("{}.tmp_{}", file_path, chaos);
        let mut output_file = File::create(&temp_file_path)?;
        forge(&mut input_file, &mut output_file)?;
        std::fs::rename(&temp_file_path, file_path)?;
        println!("Data encrypted and written to file: {}", file_path);
    }

    Ok(())
}
