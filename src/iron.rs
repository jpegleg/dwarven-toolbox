use zeroize::Zeroize;

#[path = "./generate.rs"]
mod generate;
use generate::utfgen;

fn main() {
    let mut ironchar = utfgen(32);
    println!("{}", &ironchar);
    ironchar.zeroize();
}
