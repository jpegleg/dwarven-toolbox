use std::env;
use std::io::{Read, Write, BufWriter, BufReader};
use std::fs::File;
use std::path::Path;

fn hexon(file_path: &str, out_path: &str) {
    let file_path = Path::new(file_path);
    let file = File::open(&file_path).expect("Failed to open the file.");
    let outfile = File::create(out_path).expect("Failed to create output file.");
    let mut output_writer = BufWriter::new(outfile);
    let mut input_reader = BufReader::new(file);
    let mut buffer = [0u8; 4096];

    loop {
        let bytes_read = input_reader.read(&mut buffer).unwrap();
        if bytes_read == 0 {
            break;
        }
        let hex_string = hex::encode(&buffer[..bytes_read]);
        let _ = output_writer.write_all(hex_string.as_bytes());
    }
    let _ = output_writer.flush();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Wrong number of args. The first arg is the file to encode to hex, while the second arg is the output hex file to write.");
        std::process::exit(1);
    }
    let data = args[1].clone();
    let sdata = String::from(data);
    let outdata = args[2].clone();
    let soutdata = String::from(outdata);
    hexon(&sdata, &soutdata);
}
