use std::env;
use std::num::ParseIntError;

#[derive(Debug)]
enum IntOrLong {
    U32(u32),
    U64(u64),
}

fn parse_u32_or_u64(input: &str) -> Result<IntOrLong, ParseIntError> {
    let number: u64 = input.parse::<u64>()?;

    if number <= u32::MAX as u64 {
        Ok(IntOrLong::U32(number as u32))
    } else {
        Ok(IntOrLong::U64(number))
    }
}

fn int_to_hex<T>(val: T) -> String
where
    T: std::fmt::UpperHex + Copy,
{
    let suffix = match std::mem::size_of::<T>() {
        4 => "u32",
        8 => "u64",
        _ => panic!("Enchant only supports u32 and u64!"),
    };

    format!("0x{:X}{}", val, suffix)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args. The arg is a u32 or u64 integer to convert to hex.");
        std::process::exit(1);
    }
    let input = args[1].clone();
    let conv = parse_u32_or_u64(&input);
    match conv {
        Ok(IntOrLong::U32(n)) => println!("{}", int_to_hex(n)),
        Ok(IntOrLong::U64(n)) => println!("{}", int_to_hex(n)),
        _ => println!("Input was not a u32 or u64...")
    }
}
