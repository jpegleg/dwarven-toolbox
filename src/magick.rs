use hex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Hex encode some data. If there are spaces, surround in double quotes. Usage: magick data");
        return;
    } 
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(stro) = arg.parse::<String>() {
            let encoded = hex::encode(stro);
            println!("{}", encoded);
        }
    } else {
        println!("No argument provided. If white space is needed, surround the input with doublequotes.\n\nUsage:\n\nmagick mydata\n\n");
    }
}

#[test]
fn test() {
    let input = "Great power in the encoding...";
    let hex = "477265617420706f77657220696e2074686520656e636f64696e672e2e2e";
    let lencoded = hex::encode(input);
    let binding = lencoded.as_bytes();
    let encoded = std::str::from_utf8(&binding).unwrap();
    assert_eq!(hex, encoded);
}
