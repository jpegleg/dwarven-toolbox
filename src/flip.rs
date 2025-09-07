fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Reverse a string. Usage: flip data");
        return;
    }
    let stri = &args[1];
    let strv = &mut stri.to_string().into_bytes();
    strv.reverse();
    let rezint = std::str::from_utf8(strv).unwrap().to_string();
    println!("{}", rezint);
}
