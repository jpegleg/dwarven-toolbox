use std::env;

fn popper(string: &str, bytes_to_cut: usize) -> String {
    let bytes = string.as_bytes();

    if bytes_to_cut >= bytes.len() {
        return string.to_string();
    }

    let cut_bytes = &bytes[bytes.len() - bytes_to_cut..];
    let remaining_bytes = &bytes[..bytes.len() - bytes_to_cut];
    let mut new_bytes = Vec::with_capacity(bytes.len());
    new_bytes.extend_from_slice(cut_bytes);
    new_bytes.extend_from_slice(remaining_bytes);
    String::from_utf8(new_bytes).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Wrong number of args. First arg is a usize for the number of bytes to pop, and the second arg is the string to pop with.");
        std::process::exit(1);
    }
    let numb = &args[1];
    let snumb = match numb.parse::<usize>() {
        Ok(length) => length,
        Err(_) => {
            eprintln!("Invalid first argument for pop length, defaulting to 1");
            1
        }
    };
    let data = &args[2];
    let retrb = popper(data, snumb);
    println!("{}", retrb);
}
