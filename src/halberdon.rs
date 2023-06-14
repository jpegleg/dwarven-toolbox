use chacha20poly1305::aead::{Aead, KeyInit};
use chacha20poly1305::Key;
use chacha20poly1305::XNonce;

use zeroize::Zeroize;

fn encrypt_string(nonce: &[u8], plaintext: &str, key: &[u8]) -> Result<Vec<u8>, String> {
    let key = Key::from_slice(key);
    let cipher = chacha20poly1305::XChaCha20Poly1305::new(key);
    let ciphertext = cipher
        .encrypt(XNonce::from_slice(nonce), plaintext.as_bytes())
        .map_err(|_| "Encryption failed")?;
    let mut output = Vec::new();
    output.extend_from_slice(nonce);
    output.extend_from_slice(&ciphertext);

    Ok(output)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 4 {
        println!("The first argument is the 24 byte hex nonce, the second is the message, and the third is the 32 byte hex key. If there is whitespace in the message input, surround in doublequotes.\n\nUsage example: halberd A132CE71B1EDB942748ABBD7 \"The great forge awaits you!\" 09578F03D7F3FBEB0769A7A8AC35893 \n\nNote: don't use the example nonce and key, use newly generated ones and never repeat the same nonce! The \"anvil\" program can be used to generate a new nonce and key.");
        std::process::exit(1);
    }

    let smessage = args[2].clone();
    
    let nonce = args[1].clone();
    let binding = String::from(nonce);
    let snonce = binding.as_bytes();
    let pos = snonce.len();
    let mut buffer = [0u8; 24];
    buffer[..pos].copy_from_slice(snonce);

    let key = args[3].clone();
    let mut skey = hex::decode(key).expect("Failed to decode provided key!");
    let mut buffer2 = [0u8; 32];
    let pos2 = skey.len();
    buffer2[..pos2].copy_from_slice(&skey);
    
    let result = encrypt_string(&mut buffer, &smessage, &mut buffer2);
    match result {
        Ok(ciphertext) => {
            println!("{}", hex::encode(ciphertext));
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }
    skey.zeroize();
}
