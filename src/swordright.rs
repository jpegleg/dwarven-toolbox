fn slash(string: String) -> String {
    let bytes = hex::decode(string).unwrap();
    let result_bytes: Vec<u8> = bytes
        .iter()
        .zip(bytes.iter())
        .map(|(&bytes, _self)| bytes >> 1)
        .collect();
    let hex_string = hex::encode(result_bytes);   
    hex_string
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: swordright <hex encoded data string>");
        return;
    }
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(hexdata) = arg.parse::<String>() {
            let sword = slash(hexdata);
            println!("{}", sword);
        }
    } 
}
