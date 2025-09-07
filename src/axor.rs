fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("XOR two whole numbers (u64) and print the resulting number. Usage: axor num1 num2");
        return;
    }
    
    let first: u64 = match args[1].parse::<u64>() {
      Ok(_) => args[1].parse::<u64>().expect("failed to parse u64"),
      _ => {
        eprintln!("Use u64 numbers in the two arguments");
        return
      }
    };

    let second: u64 = match args[2].parse::<u64>() {
      Ok(_) => args[1].parse::<u64>().expect("failed to parse u64"),
      _ => {
        eprintln!("Use u64 numbers in the two arguments");
        return
      }
    };

    let axor = first ^ second;
    println!("{}", axor);
}
