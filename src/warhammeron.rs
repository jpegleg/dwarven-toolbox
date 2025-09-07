use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::env;
use zeroize::Zeroize;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("The first argument is the 32 byte encoded (16 bytes decoded) hex IV, the second is the message, and the third is the 64 byte encoded (32 byte decoded) hex key. If there is whitespace in the message input, surround in doublequotes.\n\nUsage example: warhammeron FD7B5B20FF8914B56A40DAC9C8507AF2 \"The great forge awaits you.\" 0ED79DAF808F10EE5652537D3C436AA07826E9677C5FBD75D6697E9119DC6C9E \n\nNote: don't use the example IV and key, use newly generated ones and never repeat the same IV! The \"anvil\" program can be used to generate a new IV and key.");
        std::process::exit(1);
    }
    let siv = &args[1];
    let iv = hex::decode(siv).expect("Failed to decode IV provided!");
    let smessage = &args[2];
    let messbind = String::from(smessage);
    let plaintext = messbind.as_bytes();
    let key = &args[3];
    let mut skey = hex::decode(key).expect("Failed to decode private key provided!");
    let cipher = Aes256Cbc::new_from_slices(&skey, &iv).unwrap();
    let pos = plaintext.len();
    let mut buffer = [0u8; 128];
    buffer[..pos].copy_from_slice(plaintext);
    let ciphertext = cipher.encrypt(&mut buffer, pos).unwrap();
    println!("\n{:?}",hex::encode(ciphertext));
    skey.zeroize();
}
