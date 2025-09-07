use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use atty::Stream;

const CHUNK_SIZE: usize = 4096;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <file_path>", args[0]);
        return Ok(());
    }

    let file_path = &args[1];
    let stdin_is_piped = !atty::is(Stream::Stdin);

    if stdin_is_piped {
        let mut input_content = Vec::new();
        io::stdin().read_to_end(&mut input_content)?;
        let temp_file_path = format!("{}.tmp", file_path);
        let mut output_file = File::create(&temp_file_path)?;
        let mut read_pos = 0;

        while read_pos < input_content.len() {
            let end = (read_pos + CHUNK_SIZE).min(input_content.len());
            let chunk = &input_content[read_pos..end];
            read_pos = end;

            for byte in chunk {
                if *byte != b'\n' && *byte != b'\r' {
                    output_file.write_all(&[*byte])?;
                }
            }
        }

        std::fs::rename(&temp_file_path, file_path)?;
        println!("Newlines and returns removed from piped data and written to file: {}", file_path);
    } else {
        let mut input_file = File::open(file_path)?;
        let temp_file_path = format!("{}.tmp", file_path);
        let mut output_file = File::create(&temp_file_path)?;

        let mut buffer = [0; CHUNK_SIZE];
        let mut newline_found = false;
        let mut read_pos = 0;
        let mut write_pos = 0;

        loop {
            let bytes_read = input_file.read(&mut buffer)?;

            if bytes_read == 0 {
                if write_pos > 0 {
                    output_file.write_all(&buffer[..write_pos])?;
                }
                break;
            }

            for i in 0..bytes_read {
                if buffer[i] == b'\n' || buffer[i] == b'\r' {
                    newline_found = true;
                } else {
                    buffer[write_pos] = buffer[read_pos];
                    write_pos += 1;
                }

                read_pos += 1;
            }

            if newline_found {
                output_file.write_all(&buffer[..write_pos])?;
                newline_found = false;
                read_pos = 0;
                write_pos = 0;
            }
        }

        std::fs::rename(&temp_file_path, file_path)?;
        println!("Newlines and returns removed from file: {}", file_path);
    }

    Ok(())
}
