fn hexor(string1: &str, string2: &str) -> String {
    let bytes1 = hex::decode(string1).unwrap();
    let bytes2 = hex::decode(string2).unwrap();
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
