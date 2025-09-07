use num_bigint::{BigInt};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("XOR two whole numbers (BigInt) and print the resulting number. Usage: gaxor num1 num2");
        return;
    }
    let first = &args[1];
    let second = &args[2];
    if let Ok(first) = first.parse::<BigInt>() {
       if let Ok(second) = second.parse::<BigInt>() {
           let axor = first ^ second;
           println!("{}", axor);
       } else {
           println!("Use two whole (BigInt) numbers for the arguments.");
       }
    } else {
        println!("Use two whole (BigInt) numbers for the arguments.");
    }
}
