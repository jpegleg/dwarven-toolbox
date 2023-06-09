use base64::{Engine as _};

fn main() {
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
