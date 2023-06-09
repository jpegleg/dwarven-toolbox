fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(first) = arg.parse::<u64>() {
            if let Some(arg2) = std::env::args().nth(2) {
                if let Ok(second) = arg2.parse::<u64>() {
                    let axor = first ^ second;
                    println!("{}", axor);
                } else {
                    println!("Invalid argument! Use two integer arguments.");
                }
            } else {
                println!("Missing argument! Use two integer arguments.");
            }
        } else {
            println!("Invalid argument! Use two integer arguments.");
        }
    } else {
        println!("No argument provided.\n\nUsage:\n\naxor 34 42\n\n");
    }
}
