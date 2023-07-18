use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Pass in a string to check the length in bytes of.");
        std::process::exit(1);
    }
    let umb = args[1].clone();
    let clen = umb.len();
    println!("{}", clen);
}
