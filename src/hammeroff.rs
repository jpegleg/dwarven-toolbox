use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::env;
use zeroize::Zeroize;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let siv = args[1].clone();
    let binding = String::from(siv);
    let iv = binding.as_bytes();
    let smessage = args[2].clone();
    let ciphertext = hex::decode(smessage).unwrap();
    let key = args[3].clone();
    let mut skey = hex::decode(key).expect("Failed to decode private key provided!");
    let cipher = Aes128Cbc::new_from_slices(&skey, &iv).unwrap();
    let mut buf = ciphertext.to_vec();
    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
    println!("\n{:?}",std::str::from_utf8(decrypted_ciphertext).unwrap());
    skey.zeroize();
}
