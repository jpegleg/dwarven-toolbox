fn slash(string: &str) -> String {
    let bytes = hex::decode(string).unwrap();
    let result_bytes: Vec<u8> = bytes
        .iter()
        .zip(bytes.iter())
        .map(|(&bytes, _self)| bytes << 1)
        .collect();
    let hex_string = hex::encode(result_bytes);
    hex_string
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: swordleft <hex encoded data string>");
        return;
    }
    let hexdata = &args[1];
    let sword = slash(hexdata);
    println!("{}", sword);
}
