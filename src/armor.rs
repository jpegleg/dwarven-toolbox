use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use base64::{Engine as _};

const CHUNK_SIZE: usize = 4096;

fn base64_encode<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let encoded_chunk = base64::engine::general_purpose::STANDARD_NO_PAD.encode(&buffer[..bytes_read]);
        output.write_all(encoded_chunk.as_bytes())?;
    }
    Ok(())
}

fn base64_decode<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        let decoded_chunk = base64::engine::general_purpose::STANDARD_NO_PAD.decode(&buffer[..bytes_read])
            .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Base64 decode error: {}", err)))?;
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
        base64_decode(&mut input_file, &mut output_file)?;
        std::fs::rename(&temp_file_path, file_path)?;
        println!("Base64 data decoded and written to file: {}", file_path);
    } else {
        let mut input_file = File::open(file_path)?;
        let temp_file_path = format!("{}.tmp", file_path);
        let mut output_file = File::create(&temp_file_path)?;
        base64_encode(&mut input_file, &mut output_file)?;
        std::fs::rename(&temp_file_path, file_path)?;

        println!("Data encoded in Base64 and written to file: {}", file_path);
    }

    Ok(())
}
