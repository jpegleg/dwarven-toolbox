use std::env;
use num_bigint::{BigInt};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 3 {
        println!("Usage: crush <number1> <number2>");
        return;
    }
    
    let number1: BigInt = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: First argument must be a valid BigInt whole number.");
            return;
        }
    };

    let number2: BigInt = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Second argument must be a valid BigInt whole number.");
            return;
        }
    };
    
    let result = number1 - number2;
    println!("{}", result);
}
