use zeroize::Zeroize;

#[path = "./generate.rs"]
mod generate;
use generate::hexgen;

fn genkey512() -> String {
  hexgen(64)
}

fn main() {
    let mut key = genkey512();
    println!("{key}");
    key.zeroize();
}
