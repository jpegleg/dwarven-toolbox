fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Reverse a string. Usage: flip data");
        return;
    } 
    if let Some(arg) = std::env::args().nth(1) {
        if let Ok(stri) = arg.parse::<String>() {
            let strv = &mut stri.to_string().into_bytes();
            strv.reverse();
            let rezint = std::str::from_utf8(strv).unwrap().to_string();
            println!("{}", rezint);
        }
    } else {
        println!("No argument provided. \n\nUsage:\n\nflip data\n\n");
    }
}
