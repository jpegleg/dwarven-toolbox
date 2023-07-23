use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use flate2::write::ZlibEncoder;
use flate2::read::ZlibDecoder;
use flate2::Compression;

const CHUNK_SIZE: usize = 4096;

fn compress_data<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut encoder = ZlibEncoder::new(output, Compression::default());
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let bytes_read = input.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        encoder.write_all(&buffer[..bytes_read])?;
    }
    encoder.finish()?;
    Ok(())
}

fn decompress_data<R: Read, W: Write>(input: &mut R, output: &mut W) -> io::Result<()> {
    let mut decoder = ZlibDecoder::new(input);
    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let bytes_read = decoder.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        output.write_all(&buffer[..bytes_read])?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path> [-u]", args[0]);
        return Ok(());
    }

    let file_path = &args[args.len() - 1];
    let decompress_flag = args.iter().any(|arg| arg == "-u");

    if decompress_flag {
        let mut input_file = File::open(file_path)?;
        let temp_file_path = format!("{}.tmp", file_path);
        let mut output_file = File::create(&temp_file_path)?;
        decompress_data(&mut input_file, &mut output_file)?;
        std::fs::rename(&temp_file_path, file_path)?;
        println!("Data decompressed and written to file: {}", file_path);
    } else {
        let mut input_file = File::open(file_path)?;
        let temp_file_path = format!("{}.tmp", file_path);
        let mut output_file = File::create(&temp_file_path)?;
        compress_data(&mut input_file, &mut output_file)?;
        std::fs::rename(&temp_file_path, file_path)?;
        println!("Data compressed and written to file: {}", file_path);
    }
    Ok(())
}
