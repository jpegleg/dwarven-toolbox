use std::env;
use base64::{Engine as _};
use ed25519_dalek::{Signature, Verifier, PublicKey};

extern crate base64;
extern crate rand;
extern crate ed25519_dalek;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("The first argument is the message to verify. If there is whitespace in the input, surround in doublequotes. The second argument is a base64 encoded ed25519 public key, and the third argument is the signature to verify. \n\nUsage: amuletoff \"The great and powerful dwarven amulet!\" 2Cq1BiSQoYc02poHHiq/J8mJs0JXtFQBsbtid+dDhRQ 3D43A341D7813FDC97E439645662C23BF2F4A520E61B54557E9E80A36C190D5C179B4D539D1AF963CCAE8971296619D7E7B502D960D2A9BF5B409118FB19C408");
        std::process::exit(1);
    }

    let input = args[1].clone();
    let public = args[2].clone();
    let signat = args[3].clone();
    let message: &[u8] = input.as_bytes();
    let upubkey = base64::engine::general_purpose::STANDARD_NO_PAD.decode(public).unwrap();
    let pubkey = PublicKey::from_bytes(&upubkey).unwrap();
    let sigproc = signat.as_bytes();
    let dehexsign = hex::decode(sigproc).unwrap();
    let esigna: Signature = Signature::from_bytes(&dehexsign).unwrap();
    let veri = pubkey.verify(message, &esigna).is_ok();
    println!("Verification: {}", veri);

}

#[test]
fn test() {
    let input = "The great and powerful dwarven amulet!";
    let public = "2Cq1BiSQoYc02poHHiq/J8mJs0JXtFQBsbtid+dDhRQ";
    let signat = "3D43A341D7813FDC97E439645662C23BF2F4A520E61B54557E9E80A36C190D5C179B4D539D1AF963CCAE8971296619D7E7B502D960D2A9BF5B409118FB19C408";
    let message: &[u8] = input.as_bytes();
    let upubkey = base64::engine::general_purpose::STANDARD_NO_PAD.decode(public).unwrap();
    let pubkey = PublicKey::from_bytes(&upubkey).unwrap();
    let sigproc = signat.as_bytes();
    let dehexsign = hex::decode(sigproc).unwrap();
    let esigna: Signature = Signature::from_bytes(&dehexsign).unwrap();
    let veri = pubkey.verify(message, &esigna).is_ok();
    assert_eq!(true, veri);
}
