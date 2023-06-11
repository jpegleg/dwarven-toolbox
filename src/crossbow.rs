use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: crossbbow <number1> <number2>");
        return;
    }

    let number1: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: First argument must be a valid f64 number");
            return;
        }
    };

    let number2: f64 = match args[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Error: Second argument must be a valid f64 number");
            return;
        }
    };

    let result = number1 * number2;
    println!("{}", result);
}
