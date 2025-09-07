use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::env;
use zeroize::Zeroize;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("The first argument is the 32 byte encoded hex (16 bytes decoded) IV, the second is the ciphertext, and the third is the 64 byte encoded hex key (32 byte decoded) hex key. \n\nUsage example: warhammeroff FD7B5B20FF8914B56A40DAC9C8507AF2 afbb343a90a4b3f29d972af35626e06039889f2a61fe578e906666fecc428674 0ED79DAF808F10EE5652537D3C436AA07826E9677C5FBD75D6697E9119DC6C9E
 ");
        std::process::exit(1);
    }
    let siv = &args[1];
    let iv = hex::decode(siv).expect("Failed to decode IV provided!");
    let smessage = &args[2];
    let ciphertext = hex::decode(smessage).unwrap();
    let key = &args[3];
    let mut skey = hex::decode(key).expect("Failed to decode private key provided!");
    let cipher = Aes256Cbc::new_from_slices(&skey, &iv).unwrap();
    let mut buf = ciphertext.to_vec();
    let decrypted_ciphertext = cipher.decrypt(&mut buf).unwrap();
    println!("\n{:?}",std::str::from_utf8(decrypted_ciphertext).unwrap());
    skey.zeroize();
}
