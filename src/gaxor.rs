use num_bigint::{BigInt};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("XOR two whole numbers (BigInt) and print the resulting number. Usage: gaxor num1 num2");
        return;
    }
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(first) = arg.parse::<BigInt>() {
            if let Some(arg2) = std::env::args().nth(2) {
                if let Ok(second) = arg2.parse::<BigInt>() {
                    let axor = first ^ second;
                    println!("{}", axor);
                } else {
                    println!("Use two whole (BigInt) numbers for the arguments.");
                }
            } else {
                println!("Use two whole (BigInt) numbers for the arguments.");
            }
        }
    }
}
