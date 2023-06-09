use uuid::Uuid;

fn main() {
    let uid = Uuid::new_v4().to_string();
    println!("{}", uid);
}
