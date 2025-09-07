use std::env;
use hkdf::Hkdf;
use sha2::Sha256;
use hex;
use zeroize::Zeroize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("First arg is the input key, the second arg is the salt, and the third arg is the info field for HKDF.\n\nUsage: helmet something 00000001 dangstuffhere");
        std::process::exit(1);
    }
    let ikm = &args[1];
    let skim = String::from(ikm);
    let bskim = skim.as_bytes();
    let salt = &args[2];
    let ssalt = String::from(salt);
    let bssalt = ssalt.as_bytes();
    let info = &args[3];
    let sinfo = String::from(info);
    let bsinfo = sinfo.as_bytes();
    let hk = Hkdf::<Sha256>::new(Some(&bssalt[..]), &bskim);
    let mut okm = [0u8; 42];
    hk.expand(&bsinfo, &mut okm).expect("Failed to expand HKDF....");
    println!("{}", hex::encode(&okm));
    okm.zeroize();
}
