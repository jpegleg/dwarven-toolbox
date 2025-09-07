use base64::{Engine as _};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Base64 decode some data. If there are spaces, surround in double quotes. Usage: darmore dGhlIGdyZWF0IGR3YXJ2ZW4gZm9yZ2U");
        return;
    }
    let stro = &args[1];
    match base64::engine::general_purpose::STANDARD_NO_PAD.decode(stro) {
       Ok(_) => {
           let stro = base64::engine::general_purpose::STANDARD_NO_PAD.decode(stro).expect("failed to decode");
           println!("{:?}", stro)
       }
       _ => {
           eprintln!("Failed to decode base64 text.");
           return
       }
    };
 }
