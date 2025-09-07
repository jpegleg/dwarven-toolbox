use base64::{Engine as _};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Base64 encode some data. If there are spaces, surround in double quotes. Usage: aremore mydata");
        return;
    }
    let stro = &args[1];
    let armor = base64::engine::general_purpose::STANDARD_NO_PAD.encode(stro);
    println!("{}", armor);
}
