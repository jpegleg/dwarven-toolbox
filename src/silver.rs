use zeroize::Zeroize;

#[path = "./generate.rs"]
mod generate;
use generate::utfgen;

fn main() {
    let mut silverchar = utfgen(64);
    println!("{}", &silverchar);
    silverchar.zeroize();
}
