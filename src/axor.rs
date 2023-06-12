fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("XOR two whole numbes (u64) and print the resulting number. Usage: axor num1 num2");
        return;
    }
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(first) = arg.parse::<u64>() {
            if let Some(arg2) = std::env::args().nth(2) {
                if let Ok(second) = arg2.parse::<u64>() {
                    let axor = first ^ second;
                    println!("{}", axor);
                } else {
                    println!("Use two whole (u64) numbers for the arguments.");
                }
            } else {
                println!("Use two whole (u64) numbers for the arguments.");
            }
        }
    }
}
