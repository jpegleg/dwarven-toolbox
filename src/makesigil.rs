use std::error::Error;
use std::io::Write;
use std::fs::File;
use rand::TryRngCore;
use rand::rngs::OsRng;

fn randombytes(x: &mut [u8], len: usize) {
    OsRng.try_fill_bytes(&mut x[..len]).expect("OS failed to provide entropy")
}

fn gen_entropy(entropy_size: usize, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut init_seed = [0u8; 32];
    let mut entropy_file = File::create(file_path)?;
    randombytes(&mut init_seed, 32);
    entropy_file.write_all(&init_seed)?;
    entropy_file.set_len(entropy_size.try_into().unwrap())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Write a 256-bit key file. The only arg is the file path.");
        return;
    }
    let file_path = &args[1];
    let _ = gen_entropy(32, file_path);
}
