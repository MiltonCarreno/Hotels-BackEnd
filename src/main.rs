use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("P1: {}, P2: {}", args[0], args[1]);
}
