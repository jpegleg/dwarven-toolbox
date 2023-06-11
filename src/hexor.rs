fn axor(string1: &str, string2: &str) -> String {
    let bytes1 = string1.as_bytes();
    let bytes2 = string2.as_bytes();
    let result_bytes: Vec<u8> = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(&byte1, &byte2)| byte1 ^ byte2)
        .collect();
    let hex_string: String = result_bytes
        .iter()
        .map(|byte| format!("{:X}", byte))
        .collect();
    
    hex_string
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("XOR two strings and print the resulting hex. Usage: hxor string1 string2");
        return;
    }
    let string1 = args[1].clone();
    let string2 = args[2].clone();
    let hexout = axor(&string1, &string2);
    println!("{}", hexout);
}