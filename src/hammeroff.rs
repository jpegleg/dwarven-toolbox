use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::env;
use zeroize::Zeroize;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("The first argument is the 16 byte hex IV, the second is the ciphertext, and the third is the 32 byte hex key. \n\nUsage example: hammeroff 5EC4E2032363E027 8de9e8e397c8cd15bfb088dd714f4b88bbd2e78ca6606f71604ddbb6300fcb27  28083A716B0BA8A85F9E0A116FF2EDB7 ");
        std::process::exit(1);
    }
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
