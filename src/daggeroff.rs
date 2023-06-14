fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Convert u64 to hex. Usage: daggeroff data");
        return;
    } 
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(stro) = arg.parse::<u64>() {
            println!("{:X}", stro);
        }
    } else {
        println!("No argument provided. \n\nUsage:\n\ndaggeroff 100\n\n");
    }
}
