use bs58;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Base58 decode some data. If there are spaces, surround in double quotes. Usage: dshielda base58data");
        return;
    }
    let stro = &args[1];
    let decoded = match bs58::decode(stro).into_vec() {
        Ok(_) => bs58::decode(stro).into_vec().expect("failed to decode"),
        _ => {
           eprintln!("Failed to decode base58 text.");
           return
        }
    };
    let stringa = match std::str::from_utf8(&decoded) {
        Ok(_) => std::str::from_utf8(&decoded).expect("failed to decode to UTF-8"),
         _ => {
           eprintln!("Failed to decode base58 text to UTF-8.");
           return
        }
    };

    println!("{}", stringa);
}
