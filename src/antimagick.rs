use hex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Hex decode some data. If there are spaces, surround in double quotes. Usage: antimagick data");
        return;
    } 
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(stro) = arg.parse::<String>() {
            let decoded = hex::decode(stro).unwrap();
            let stringa = std::str::from_utf8(&decoded).unwrap();
            println!("{}", stringa);
        } 
    } else {
        println!("No argument provided. If white space is needed, surround the input with doublequotes.\n\nUsage:\n\nantimagick mydata\n\n");
    }
}
