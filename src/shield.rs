use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use bs58;

const CHUNK_SIZE: usize = 4096;

fn base58_encode<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let encoded_chunk = bs58::encode(&buffer[..bytes_read]).into_vec();
        output.write_all(&encoded_chunk)?;
    }
    Ok(())
}

fn base58_decode<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let decoded_chunk = bs58::decode(&buffer[..bytes_read]).into_vec().unwrap();
        output.write_all(&decoded_chunk)?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path> [-d]", args[0]);
        return Ok(());
    }
    let file_path = &args[1];
    let decode_flag = args.iter().any(|arg| arg == "-d");
    if decode_flag {
        let mut input_file = File::open(file_path)?;
        let temp_file_path = format!("{}.tmp", file_path);
        let mut output_file = File::create(&temp_file_path)?;
        base58_decode(&mut input_file, &mut output_file)?;
        std::fs::rename(&temp_file_path, file_path)?;
        println!("Base58 data decoded and written to file: {}", file_path);
    } else {
        let mut input_file = File::open(file_path)?;
        let temp_file_path = format!("{}.tmp", file_path);
        let mut output_file = File::create(&temp_file_path)?;
        base58_encode(&mut input_file, &mut output_file)?;
        std::fs::rename(&temp_file_path, file_path)?;
        println!("Data encoded in Base58 and written to file: {}", file_path);
    }
    Ok(())
}
