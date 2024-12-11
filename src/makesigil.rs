use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::os::unix::fs::PermissionsExt;
use std::fs::set_permissions;
use rand::rngs::OsRng;
use base64::{Engine as _};
use ed25519_dalek::{Keypair, PUBLIC_KEY_LENGTH};

extern crate base64;
extern crate ed25519_dalek;

use zeroize::Zeroize;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("{{\"ERROR\": \"Wrong number of args. The only arg is a file path write a new ed25519 keypair binary to.\"}}");
        std::process::exit(1);
    }
    let data = args[1].clone();
    let sdata = String::from(data);
    let datafile_path = Path::new(&sdata);
    let mut datafile = File::create(&datafile_path).expect("Failed to open the file");
    let mut databytes = Vec::new();
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    let secret_key_bytes: Vec<u8> = keypair.secret.to_bytes().to_vec();
    let public_key_bytes: Vec<u8> = keypair.public.to_bytes().to_vec();
    databytes.extend_from_slice(&secret_key_bytes);
    databytes.extend_from_slice(&public_key_bytes);

    let public_key_bytes: [u8; PUBLIC_KEY_LENGTH] = keypair.public.to_bytes();
    let pub64 = base64::engine::general_purpose::STANDARD_NO_PAD.encode(public_key_bytes);

    datafile.write_all(&databytes).expect("Failed to read the file");
    set_permissions(&datafile_path, PermissionsExt::from_mode(0o600)).unwrap();
    println!("Public key: {:?}\nKeypair binary created at: {:?}", pub64, datafile_path);
    databytes.zeroize();

}
