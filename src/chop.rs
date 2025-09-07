use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Wrong number of args. First arg is a usize for the number of bytes to cut, and the second arg is the string to cut.");
        std::process::exit(1);
    }
    let numb = &args[1];
    let snumb = match numb.parse::<usize>() {
        Ok(length) => length,
        Err(_) => {
            eprintln!("Invalid first argument for cut length, defaulting to 1");
            1
        }
    };
    let data = &args[2];
    let to_cut = snumb.min(data.len());
    let cut = &data[..to_cut];
    println!("{}", cut);
}
