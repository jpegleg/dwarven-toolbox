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
        } else {
            println!("Invalid argument! Use a single argument.");
        }
    } else {
        println!("No argument provided. If white space is needed, surround the input with doublequotes.\n\nUsage:\n\nmagick mydata\n\n");
    }
}
