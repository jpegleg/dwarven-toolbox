use bs58;

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(stro) = arg.parse::<String>() {
            let decoded = bs58::decode(stro).into_vec().unwrap();
            let stringa = std::str::from_utf8(&decoded).unwrap(); 
            println!("{}", stringa);
        } else {
            println!("Invalid argument! Use a single argument.");
        }
    } else {
        println!("No argument provided. If white space is needed, surround the input with doublequotes.\n\nUsage:\n\ndshielda 3QTA1HZx89uB6L2x4aLax8Cet3Hm9zQk\n\n");
    }
}
