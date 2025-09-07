use hex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Hex encode some data. If there are spaces, surround in double quotes. Usage: magick data");
        return;
    }
    let stro = &args[1];
    let encoded = hex::encode(stro);
    println!("{}", encoded);
}
