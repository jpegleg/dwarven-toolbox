use zeroize::Zeroize;

#[path = "./generate.rs"]
mod generate;
use generate::hexgen;

fn main() {
    let mut ironchar = hexgen(32);
    println!("{}", &ironchar);
    ironchar.zeroize();
}
