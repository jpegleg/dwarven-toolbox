use hex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Hex decode some data. If there are spaces, surround in double quotes. Usage: antimagick data");
        return;
    }
    let stro = &args[1];
    let decoded = hex::decode(stro).unwrap();
    let stringa = std::str::from_utf8(&decoded).unwrap();
    println!("{}", stringa);
}
