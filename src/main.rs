use std::env;

fn main() {
    let file: Vec<String> = env::args().collect();
    println!("{:?}", file);
}
