use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::env;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let siv = args[1].clone();
    let binding = String::from(siv);
    let iv = binding.as_bytes();
    let smessage = args[2].clone();
    let messbind = String::from(smessage);
    let plaintext = messbind.as_bytes();
    let key = args[3].clone();
    let skey = hex::decode(key).expect("Failed to decode private key provided!");
    let cipher = Aes128Cbc::new_from_slices(&skey, &iv).unwrap();
    let pos = plaintext.len();
    let mut buffer = [0u8; 128];
    buffer[..pos].copy_from_slice(plaintext);
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();
    println!("\n{:?}",hex::encode(ciphertext));
}
