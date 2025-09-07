use num_bigint::BigUint;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Convert BigUint to hex. Usage: daggeroff data");
        return;
    }
    let arg = &args[1];
    match arg.parse::<BigUint>() {
       Ok(_) => {
           let stro = arg.parse::<BigUint>().expect("failed to parse BigUint");
           println!("{:X}", stro)
       }
       _ => {
           eprintln!("Failed to decode BigUint.");
           return
       }
    };
}
