fn hexor(string1: &str, string2: &str) -> String {
    let bytes1 = match hex::decode(string1) {
        Ok(_) => hex::decode(string1).expect("failed to decode hex"),
        _ => {
           eprintln!("Error decoding hex.");
           return Default::default()
        }
    };
    let bytes2 = match hex::decode(string2) {
        Ok(_) => hex::decode(string2).expect("failed to decode hex"),
        _ => {
           eprintln!("Error decoding hex.");
           return Default::default()
        }
    };
    let result_bytes: Vec<u8> = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&byte1, &byte2)| byte1 ^ byte2)
        .collect();
    let hex_string = hex::encode(result_bytes);
    hex_string
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("XOR two strings and print the resulting hex. Usage: hexor string1 string2");
        return;
    }
    let string1 = &args[1];
    let string2 = &args[2];
    let hexout = hexor(string1, string2);
    println!("{}", hexout);
}
