use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::Key;
use chacha20poly1305::Nonce;

use zeroize::Zeroize;

fn decrypt_string(nonce: &[u8], cipher_hex: &str, key: &[u8; 32]) -> Result<String, String> {
    let ciphertext = hex::decode(cipher_hex).map_err(|_| "Failed to decode ciphertext hex")?;
    let nonce_size = 12;
    if ciphertext.len() < nonce_size {
        return Err("Invalid ciphertext".to_string());
    }

    let (received_nonce, received_ciphertext) = ciphertext.split_at(nonce_size);
    let nonce = if nonce.is_empty() {
        received_nonce
    } else {
        nonce
    };

    let key = Key::from_slice(key);
    let cipher = chacha20poly1305::ChaCha20Poly1305::new(key);

    let plaintext = cipher
        .decrypt(Nonce::from_slice(nonce), received_ciphertext)
        .map_err(|_| "Decryption failed")?;

    let plaintext = String::from_utf8(plaintext).map_err(|_| "Invalid UTF-8")?;

    Ok(plaintext)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("The first argument is the 12 byte hex nonce, the second is the hex encoded ciphertext, and the third is the 32 byte hex key. If there is whitespace in the message input, surround in doublequotes.\n\nUsage example: mattockoff 4450501658F5 3434353035303136353846356cafc158a33dd76031e868d6c4b88a6741f85b9114118cdca7b9b5b039ca1318ce982eff0abf0ea1de3ad6 09578F03D7F3FBEB0769A7A8AC358936 \n\nNote: don't use the example nonce and key, use newly generated ones and never repeat the same! The \"anvil\" program can be used to generate a new nonce and key.");
        std::process::exit(1);
    }

    let smessage = args[2].clone();
    
    let nonce = args[1].clone();
    let binding = String::from(nonce);
    let snonce = binding.as_bytes();
    let pos = snonce.len();
    let mut buffer = [0u8; 12];
    buffer[..pos].copy_from_slice(snonce);

    let key = args[3].clone();
    let mut skey = hex::decode(key).expect("Failed to decode provided key!");
    let mut buffer2 = [0u8; 32];
    let pos2 = skey.len();
    buffer2[..pos2].copy_from_slice(&skey);
    
    let result = decrypt_string(&mut buffer, &smessage, &mut buffer2);
    match result {
        Ok(output) => {
            println!("{}", output);
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
    skey.zeroize();
}
