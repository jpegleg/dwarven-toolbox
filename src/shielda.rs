use bs58;

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(stro) = arg.parse::<String>() {
            let encoded = bs58::encode(stro).into_string();
            println!("{}", encoded);
        } else {
            println!("Invalid argument! Use a single argument.");
        }
    } else {
        println!("No argument provided. If white space is needed, surround the input with doublequotes.\n\nUsage:\n\nshielda mydata\n\n");
    }
}
