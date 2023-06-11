pub fn chkarg(argument: &f64) {
    let argument_string = argument.to_string();
    if argument_string.len() > 16 {
        eprintln!("Error: Argument longer than 16 bytes not supported!");
        std::process::exit(1);
    }
}
