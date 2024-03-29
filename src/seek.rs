use std::env;

fn grep(filter: &str, input: &str) -> String {
    let mut result = String::new();
    let mut found = false;

    for i in 0..input.len() {
        if input[i..].starts_with(filter) {
            result.push_str(&input[i..i + filter.len()]);
            found = true;
            break;
        }
    }

    if !found {
        panic!("Pattern '{}' not found in input.", filter);
    }

    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Wrong number of args. The first arg is a filter to seek and the second arg is the string to seek from.");
        std::process::exit(1);
    }
    let filt = args[1].clone();
    let sfilt = String::from(filt);
    let data = args[2].clone();
    let sdata = String::from(data);
    let retrb = grep(&sfilt, &sdata);
    println!("{}", retrb);
}
