use std::env;
use std::io::Read;
use std::fs::File;
use std::path::Path;

fn inspect(file_path: &str) {
    let file_path = Path::new(file_path);
    let mut file = File::open(&file_path).expect("Failed to open the file.");

    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("Failed to read the file");

    let num_bytes = bytes.len();
    let num_bits = num_bytes * 8;

    print!("Byte vector: {:?}\n\n", bytes);
    print!("Each byte as hex: ");
    for byte in &bytes {
        print!("{:X} ", byte);
    }
    println!();
    println!("Number of bytes: {}", num_bytes);
    println!("Number of bits: {}", num_bits);

}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args. The only arg is a file to inspect as hex and count the bits and bytes.");
        std::process::exit(1);
    }
    let data = args[1].clone();
    let sdata = String::from(data);
    inspect(&sdata);
}
