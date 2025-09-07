use bs58;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Base58 encode some data. If there are spaces, surround in double quotes. Usage: shielda data");
        return;
    }
    let stro = &args[1];
    let encoded = bs58::encode(stro).into_string();
    println!("{}", encoded);
}
