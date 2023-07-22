use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

const CHUNK_SIZE: usize = 4096;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }

    let file_path = &args[args.len() - 1];
    let mut file_content = Vec::new();
    let mut input_file = File::open(file_path)?;
    input_file.read_to_end(&mut file_content)?;

    for byte in &mut file_content {
        *byte = !*byte;
    }

    let temp_file_path = format!("{}.tmp", file_path);
    let mut output_file = File::create(&temp_file_path)?;
    let mut read_pos = 0;

    while read_pos < file_content.len() {
        let end = (read_pos + CHUNK_SIZE).min(file_content.len());
        let chunk = &file_content[read_pos..end];
        read_pos = end;

        output_file.write_all(chunk)?;
    }

    std::fs::rename(&temp_file_path, file_path)?;
    println!("Bits flipped and written to file: {}", file_path);

    Ok(())
}
