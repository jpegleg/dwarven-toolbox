use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::env;
use zeroize::Zeroize;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("The first argument is the 16 byte hex IV, the second is the message, and the third is the 32 byte encoded (16 bytes decoded) hex key. If there is whitespace in the message input, surround in doublequotes.\n\nUsage example: hammeron 5EC4E2032363E027 \"The great forge awaits you!\" 28083A716B0BA8A85F9E0A116FF2EDB7 \n\nNote: don't use the example IV and key, use newly generated ones and never repeat the same IV! The \"anvil\" program can be used to generate a new IV and key.");
        std::process::exit(1);
    }
    let siv = args[1].clone();
    let binding = String::from(siv);
    let iv = binding.as_bytes();
    let smessage = args[2].clone();
    let messbind = String::from(smessage);
    let plaintext = messbind.as_bytes();
    let key = args[3].clone();
    let mut skey = hex::decode(key).expect("Failed to decode private key provided!");
    let cipher = Aes128Cbc::new_from_slices(&skey, &iv).unwrap();
    let pos = plaintext.len();
    let mut buffer = [0u8; 128];
    buffer[..pos].copy_from_slice(plaintext);
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();
    println!("\n{:?}",hex::encode(ciphertext));
    skey.zeroize();
}
