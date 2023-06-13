use base64::{Engine as _};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Base64 encode some data. If there are spaces, surround in double quotes. Usage: aremore mydata");
        return;
    }
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(stro) = arg.parse::<String>() {
            let armor = base64::engine::general_purpose::STANDARD_NO_PAD.encode(stro);
            println!("{}", armor);
        } else {
            println!("Invalid argument! Use a single string argument.");
        }
    } else {
        println!("No argument provided. If white space is needed, surround the input with doublequotes.\n\nUsage:\n\narmore mydata\n\n");
    }
}
