use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::prelude::*;
use hex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Gzip compress some data. If there are spaces, surround in double quotes. Usage: box data");
        return;
    } 
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(stro) = arg.parse::<String>() {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            let rawdata = stro.as_bytes();
            let _self = encoder.write_all(rawdata);
            let gzdata = encoder.finish().unwrap();
            let encoded = hex::encode(gzdata);
            println!("{}", encoded);
        }
    } else {
        println!("No argument provided. If white space is needed, surround the input with doublequotes.\n\nUsage:\n\nbox mydata\n\n");
    }
}
