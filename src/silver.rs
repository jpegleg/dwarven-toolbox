use zeroize::Zeroize;

#[path = "./generate.rs"]
mod generate;
use generate::hexgen;

fn main() {
    let mut silverchar = hexgen(64);
    println!("{}", &silverchar);
    silverchar.zeroize();
}
