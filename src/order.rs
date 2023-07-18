use std::env;

fn sort_bytes(string: &str) -> String {
    let mut bytes = string.bytes().collect::<Vec<u8>>();
    bytes.sort_unstable();
    let mut result = String::with_capacity(string.len());
    let mut prev_byte: Option<u8> = None;
    let mut count = 0;

    for byte in bytes {
        if Some(byte) == prev_byte {
            count += 1;
        } else {
            if let Some(prev) = prev_byte {
                result.push_str(&format!("{}{}", count, char::from(prev)));
            }
            count = 1;
            prev_byte = Some(byte);
        }
    }

    if let Some(byte) = prev_byte {
        result.push_str(&format!("{}{}", count, char::from(byte)));
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args. The only arg is a string to sort the bytes from.");
        std::process::exit(1);
    }
    let data = args[1].clone();
    let sdata = String::from(data);
    let retrb = sort_bytes(&sdata);
    println!("{}", retrb);
}
