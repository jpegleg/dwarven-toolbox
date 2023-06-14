use hex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Convert hex data to a u64 integer. Usage: daggeron data");
        return;
    } 
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(stro) = arg.parse::<String>() {
            let decoded = hex::decode(stro).unwrap();
            let mut buf = [0u8; 8];
            let len = 8.min(decoded.len());
            buf[..len].copy_from_slice(&decoded[..len]);
            let rezint = u64::from_le_bytes(buf);
            println!("{}", rezint);
        }
    } else {
        println!("No argument provided. \n\nUsage:\n\ndaggeron hexdata\n\n");
    }
}
