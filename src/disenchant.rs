use std::env;

#[derive(Debug)]
enum IntOrLong {
    U32(u32),
    U64(u64),
}

impl IntOrLong {
    fn to_decimal_string(&self) -> String {
        match self {
            IntOrLong::U32(n) => n.to_string(),
            IntOrLong::U64(n) => n.to_string(),
        }
    }
}

fn parse_hex_int(input: &str) -> Result<IntOrLong, String> {
    if !input.starts_with("0x") {
        return Err(format!("Expected '0x' prefix, got: {}", input));
    }

    let suffix_len = 3;
    if input.len() < 2 + suffix_len {
        return Err(format!("String too short to contain '0x' and suffix: {}", input));
    }

    let (maybe_hex_part, maybe_suffix) = input.split_at(input.len() - suffix_len);

    match maybe_suffix {
        "u32" => {
            let hex_part = &maybe_hex_part[2..];
            let parsed_val = u32::from_str_radix(hex_part, 16)
                .map_err(|e| format!("Failed to parse u32 from `{}`: {}", hex_part, e))?;
            Ok(IntOrLong::U32(parsed_val))
        }
        "u64" => {
            let hex_part = &maybe_hex_part[2..];
            let parsed_val = u64::from_str_radix(hex_part, 16)
                .map_err(|e| format!("Failed to parse u64 from `{}`: {}", hex_part, e))?;
            Ok(IntOrLong::U64(parsed_val))
        }
        _ => Err(format!("Expected suffix 'u32' or 'u64', got: {}", maybe_suffix)),
    }
}

fn parse_hex_to_decimal_str(input: &str) -> Result<String, String> {
    let int_val = parse_hex_int(input)?;
    Ok(int_val.to_decimal_string())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Wrong number of args. The arg is a u32 or u64 integer to convert to hex.");
        std::process::exit(1);
    }
    let input = &args[1];
    let rezult = match parse_hex_to_decimal_str(input) {
        Ok(out) => out,
        Err(e) => {
            eprintln!("ERROR: {e}");
            return
        }
    };
    println!("{}", &rezult);
}
