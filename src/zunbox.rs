use flate2::read::ZlibDecoder;
use std::io::prelude::*;
use hex;

fn hex_decode_gzip(hex_string: &str) -> Result<String, String> {
    let decoded_bytes = hex::decode(hex_string).map_err(|_| "Failed to decode hex string")?;
    let mut decoder = ZlibDecoder::new(&decoded_bytes[..]);
    let mut decompressed_data = Vec::new();
    decoder
        .read_to_end(&mut decompressed_data)
        .map_err(|_| "Failed to decompress zlib data")?;

    let hex_encoded = hex::encode(&decompressed_data);
    Ok(hex_encoded)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Zlib decompress some data. If there are spaces, surround in double quotes. Usage: zunbox 789c0bc94855482f4a4d2c5128c82f4f2d52c84f5348294f2c2a4bcd53c8cd4ccf28510400cc640c08");
        return;
    }
    let stro = &args[1];
    let encoded = match hex_decode_gzip(&stro) {
        Ok(_) => hex_decode_gzip(&stro).expect("failed to decode"),
        _ => {
           eprintln!("Failed to decompres data.");
           return
        }
    };
    println!("{}", encoded);
}
