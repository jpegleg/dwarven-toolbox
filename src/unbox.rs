use flate2::read::GzDecoder;
use std::io::prelude::*;
use hex;

fn hex_decode_gzip(hex_string: &str) -> Result<String, String> {
    let decoded_bytes = hex::decode(hex_string).map_err(|_| "Failed to decode hex string")?;
    let mut decoder = GzDecoder::new(&decoded_bytes[..]);
    let mut decompressed_data = Vec::new();
    decoder
        .read_to_end(&mut decompressed_data)
        .map_err(|_| "Failed to decompress Gzip data")?;

    let hex_encoded = hex::encode(&decompressed_data);
    Ok(hex_encoded)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Gzip decompress some data. If there are spaces, surround in double quotes. Usage: unbox 1f8b08000000000000ff0bc9485548294f2c2a4bcd53c8cd4ccf2851040000b9c2c112000000");
        return;
    }
    let stro = &args[1];
    let encoded = hex_decode_gzip(&stro).unwrap();
    println!("{}", encoded);
}
