use zeroize::Zeroize;

#[path = "./generate.rs"]
mod generate;
use generate::hexgen;

fn main() {
    let mut goldchar = hexgen(96);
    println!("{}", &goldchar);
    goldchar.zeroize();
}
