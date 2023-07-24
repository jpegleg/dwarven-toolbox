use std::env;
use zeroize::Zeroize;
use aes_gcm_siv::{
    aead::{Aead, KeyInit, OsRng},
    Aes256GcmSiv, Nonce 
};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Uniform;
use std::iter;

fn genon() -> String {
    let mut rng = StdRng::from_entropy();
    let genonchar: String = iter::repeat(())
        .map(|()| {
            let char_range = Uniform::from(32..127);
            rng.sample(char_range) as u8 as char
        })
        .take(12)
        .collect();
    genonchar
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("The only arg is a string to encrypt, 127 bytes or less.\nUsage: steelhammeron somedatastring");
        std::process::exit(1);
    }
    let siv = genon();
    let binding = String::from(siv);
    let bnon = &binding.as_bytes();
    let nonce = Nonce::from_slice(bnon);
    let smessage = args[1].clone();
    let messbind = String::from(smessage);
    let plaintext = messbind.as_bytes();
    let pos = plaintext.len();
    let mut buffer = [0u8; 128];
    buffer[..pos].copy_from_slice(plaintext);

    let mut key = Aes256GcmSiv::generate_key(&mut OsRng);
    let cipher = Aes256GcmSiv::new(&key);
    let ciphertext = cipher.encrypt(nonce.into(), plaintext.as_ref());

    let hexcipher = hex::encode(ciphertext.unwrap());
    println!("\nCiphertext: {:?}", hexcipher);
    println!("Nonce: {:?}", hex::encode(binding));
    println!("Key: {:?}", hex::encode(&key));
    key.zeroize();
}
