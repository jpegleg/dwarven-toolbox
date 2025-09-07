use zeroize::Zeroize;

#[path = "./generate.rs"]
mod generate;
use generate::utfgen;

fn main() {
    let mut goldchar = utfgen(96);
    println!("{}", &goldchar);
    goldchar.zeroize();
}
