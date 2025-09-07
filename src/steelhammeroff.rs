use std::env;
use aes_gcm_siv::{
    aead::{Aead, KeyInit},
    Aes256GcmSiv, Nonce
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("The first arg is the nonce, the second is the ciphertext, and the third is the key. Example usage:\nsteelhammeroff 7e5e3b67256366522c316956 e5325f3aa1c5324cf0dd762c0dd82950e9e40f46cfd15f6a983a908025e3c365c8f51330715f6900a690ea003a5c76e349654883096eb48638ddec 10bed3c8e2c27f8abef1e7d0fa129c79089dbb39894f628a4008eb45521c4ec7
");
        std::process::exit(1);
    }
    let siv = &args[1];
    let binding = String::from(siv);
    let bnon = hex::decode(binding).unwrap();
    let nonce = Nonce::from_slice(&bnon);
    let smessage = &args[2];
    let akey = &args[3];
    let mut bkey = hex::decode(akey).unwrap();
    let ciphertext = hex::decode(smessage).unwrap();
    let pos = ciphertext.len();
    let mut buffer = [0u8; 128];
    buffer[..pos].copy_from_slice(&ciphertext);

    let cipher = Aes256GcmSiv::new_from_slice(&mut bkey);
    let plaintext = cipher.expect("Failed to decrypt ciphertext.").decrypt(nonce, ciphertext.as_ref());
    let printtext = String::from_utf8(plaintext.unwrap()).map_err(|_| "Invalid UTF-8").unwrap();
    println!("{}", printtext);
}
