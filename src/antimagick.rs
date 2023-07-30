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

#[test]
fn test() {
    let input = "Great power in the encoding...";
    let hex = "477265617420706f77657220696e2074686520656e636f64696e672e2e2e";
    let lencoded = hex::decode(hex);
    let binding = lencoded.unwrap();
    let decoded = std::str::from_utf8(&binding).unwrap();
    assert_eq!(input, decoded);
}
