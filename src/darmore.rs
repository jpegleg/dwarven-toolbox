use std::str;
use base64::{Engine as _};

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(stro) = arg.parse::<String>() {
            let darmor = base64::engine::general_purpose::STANDARD_NO_PAD.decode(stro).unwrap();
            let text = str::from_utf8(&darmor).unwrap();
            println!("{:?}", text);
        } else {
            println!("Invalid argument! Use a single string argument.");
        }
    } else {
        println!("No argument provided. If white space is needed, surround the input with doublequotes.\n\nUsage:\n\ndarmore dGhlIGdyZWF0IGR3YXJ2ZW4gZm9yZ2U\n\n");
    }
}
